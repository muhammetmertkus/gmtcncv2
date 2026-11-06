//! Opt-in, simulator-only control client. Socket I/O never runs on the UI thread.
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use linc_diag::{client::Client, discovery};
use serde_json::json;

#[derive(Clone, Default)]
pub(super) struct State {
    pub ready: bool,
    pub connection_generation: u64,
    pub estop: bool,
    pub protect: bool,
    pub probe_enabled: bool,
    pub feed: f64,
    pub rapid: f64,
    pub spindle: f64,
    pub disconnected_buttons: Vec<String>,
    pub active_leds: Vec<String>,
    pub checked_at: Option<Instant>,
}

#[derive(Clone, Debug)]
pub(super) enum Command {
    Estop(bool),
    Power(bool),
    Protect(bool),
    Mode(&'static str),
    Feed(f64),
    Rapid(f64),
    Spindle(f64),
    CycleStart,
    CycleStop,
}

impl Command {
    fn payload(&self) -> serde_json::Value {
        match self {
            Self::Estop(value) => json!({"estop": value}),
            Self::Power(value) => json!({"power_key": if *value { "on" } else { "off" }}),
            Self::Protect(value) => json!({"program_protect": value}),
            Self::Mode(value) => json!({"mode": value}),
            Self::Feed(value) => json!({"feed_override": value}),
            Self::Rapid(value) => json!({"rapid_override": value}),
            Self::Spindle(value) => json!({"spindle_override": value}),
            Self::CycleStart => json!({"press": ["cycle_start"], "duration_ms": 120}),
            Self::CycleStop => json!({"press": ["cycle_stop"], "duration_ms": 120}),
        }
    }
    fn method(&self) -> &'static str {
        match self {
            Self::CycleStart | Self::CycleStop => "diag.sim_buttons",
            _ => "diag.sim_set",
        }
    }
}

#[derive(Default)]
struct HeldKeys {
    keys: Vec<&'static str>,
    refreshed: Option<Instant>,
    connection_generation: u64,
}

pub(super) struct PanelPreviewControl {
    tx: mpsc::Sender<Command>,
    state: Arc<Mutex<State>>,
    held: Arc<Mutex<HeldKeys>>,
}

impl PanelPreviewControl {
    pub fn new(profile: String) -> Self {
        Self::in_dir(profile, discovery::runtime_dir())
    }

    pub(super) fn in_dir(profile: String, dir: std::path::PathBuf) -> Self {
        let (tx, rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(State::default()));
        let shared = state.clone();
        let held = Arc::new(Mutex::new(HeldKeys::default()));
        let held_for_worker = held.clone();
        thread::spawn(move || worker(profile, dir, rx, shared, held_for_worker));
        Self { tx, state, held }
    }

    pub fn state(&self) -> State {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Replace the current pointer/touch holds. Stale UI frames are never renewed.
    pub fn set_held(&self, keys: Vec<&'static str>, connection_generation: u64) {
        let mut held = self.held.lock().unwrap_or_else(|e| e.into_inner());
        held.keys = keys;
        held.refreshed = Some(Instant::now());
        held.connection_generation = connection_generation;
    }

    /// Only a fresh, matching simulator state may accept a request.
    pub fn send(&self, command: Command) {
        let state = self.state();
        if state.ready
            && state
                .checked_at
                .is_some_and(|at| at.elapsed() < Duration::from_millis(750))
        {
            let _ = self.tx.send(command);
        }
    }
}

fn worker(
    profile: String,
    dir: std::path::PathBuf,
    rx: mpsc::Receiver<Command>,
    shared: Arc<Mutex<State>>,
    held: Arc<Mutex<HeldKeys>>,
) {
    let mut connection_generation = 0u64;
    loop {
        let instance = discovery::read_instance(&dir, "panel").ok();
        let mut client = instance
            .filter(|i| i.build.backend == "sim" && i.profile == profile)
            .and_then(|i| Client::connect(&i.socket).ok())
            .map(|client| client.with_timeout(250));
        if client.is_some() {
            connection_generation = connection_generation.wrapping_add(1);
        }
        let mut previous: Vec<&'static str> = Vec::new();
        while let Some(conn) = client.as_mut() {
            let response = conn.call("diag.sim_state", json!({}));
            let result = response.ok().and_then(|value| simulator_estop(&value));
            let Some((
                estop,
                protect,
                probe_enabled,
                feed,
                rapid,
                spindle,
                disconnected_buttons,
                active_leds,
            )) = result
            else {
                break;
            };
            *shared.lock().unwrap_or_else(|e| e.into_inner()) = State {
                ready: true,
                connection_generation,
                estop,
                protect,
                probe_enabled,
                feed,
                rapid,
                spindle,
                disconnected_buttons,
                active_leds,
                checked_at: Some(Instant::now()),
            };
            // A hold is a renewable short lease, never an unbounded press.
            // A frozen, hidden or dead UI stops refreshing within 200 ms;
            // the panel service then releases its lease independently.
            let current = {
                let held = held.lock().unwrap_or_else(|e| e.into_inner());
                if held.connection_generation == connection_generation
                    && held
                        .refreshed
                        .is_some_and(|at| at.elapsed() < Duration::from_millis(200))
                {
                    held.keys.clone()
                } else {
                    Vec::new()
                }
            };
            let released: Vec<_> = previous
                .iter()
                .filter(|key| !current.contains(key))
                .copied()
                .collect();
            if !released.is_empty()
                && conn
                    .call("diag.sim_buttons", json!({"release": released}))
                    .is_err()
            {
                break;
            }
            let mut failed = false;
            for key in &current {
                if conn
                    .call(
                        "diag.sim_buttons",
                        json!({"press": [key], "duration_ms": 300}),
                    )
                    .is_err()
                {
                    failed = true;
                    break;
                }
            }
            if failed {
                break;
            }
            previous = current;
            match rx.recv_timeout(Duration::from_millis(70)) {
                Ok(command) => {
                    if conn.call(command.method(), command.payload()).is_err() {
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        }
        *shared.lock().unwrap_or_else(|e| e.into_inner()) = State::default();
        // Clear local holds across reconnect, even if the UI has stopped.
        *held.lock().unwrap_or_else(|e| e.into_inner()) = HeldKeys::default();
        // Discard every request issued on the old connection. A later
        // simulator process must never receive commands from an earlier one.
        while rx.try_recv().is_ok() {}
        match rx.recv_timeout(Duration::from_millis(500)) {
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
            _ => {} // Never replay commands after a disconnect.
        }
    }
}

/// Accept only fresh responses from a connected simulator without a link error.
fn simulator_estop(
    value: &serde_json::Value,
) -> Option<(bool, bool, bool, f64, f64, f64, Vec<String>, Vec<String>)> {
    if value.get("stale")?.as_bool()? {
        return None;
    }
    let data = value.get("data")?;
    let state = data.get("state")?;
    if !data.get("armed")?.as_bool()?
        || !state.get("connected")?.as_bool()?
        || state.get("error")?.as_bool()?
    {
        return None;
    }
    Some((
        state.get("estop")?.as_bool()?,
        state.get("program_protect")?.as_bool()?,
        state.get("probe_enabled")?.as_bool()?,
        state.get("feed_override")?.as_f64()?,
        state.get("rapid_override")?.as_f64()?,
        state.get("spindle_override")?.as_f64()?,
        state
            .get("disconnected_buttons")?
            .as_array()?
            .iter()
            .map(|value| value.as_str().map(str::to_owned))
            .collect::<Option<Vec<_>>>()?,
        state
            .get("active_leds")?
            .as_array()?
            .iter()
            .map(|value| value.as_str().map(str::to_owned))
            .collect::<Option<Vec<_>>>()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn panel_controls_map_to_the_simulator_protocol() {
        assert_eq!(Command::Estop(true).payload(), json!({"estop": true}));
        assert_eq!(Command::Power(true).payload(), json!({"power_key": "on"}));
        assert_eq!(Command::CycleStart.method(), "diag.sim_buttons");
        assert_eq!(
            Command::CycleStop.payload(),
            json!({"press": ["cycle_stop"], "duration_ms": 120})
        );
        assert_eq!(
            Command::Protect(false).payload(),
            json!({"program_protect": false})
        );
        assert_eq!(Command::Mode("JOG").payload(), json!({"mode": "JOG"}));
        assert_eq!(Command::Feed(1.5).payload(), json!({"feed_override": 1.5}));
        assert_eq!(
            Command::Rapid(0.5).payload(),
            json!({"rapid_override": 0.5})
        );
        assert_eq!(
            Command::Spindle(1.2).payload(),
            json!({"spindle_override": 1.2})
        );
    }

    #[test]
    fn accepts_only_fresh_connected_armed_simulator_state() {
        let mut value = json!({"stale": false, "data": {"armed": true, "state": {"connected": true, "error": false, "estop": true, "program_protect": false, "probe_enabled": false, "feed_override": 1.0, "rapid_override": 1.0, "spindle_override": 1.0, "disconnected_buttons": [], "active_leds": []}}});
        assert_eq!(
            simulator_estop(&value),
            Some((true, false, false, 1.0, 1.0, 1.0, vec![], vec![]))
        );
        value["stale"] = json!(true);
        assert_eq!(simulator_estop(&value), None);
        value["stale"] = json!(false);
        value["data"]["state"]["connected"] = json!(false);
        assert_eq!(simulator_estop(&value), None);
        value["data"]["state"]["connected"] = json!(true);
        value["data"]["armed"] = json!(false);
        assert_eq!(simulator_estop(&value), None);
    }
    #[test]
    fn simulator_socket_releases_expired_jog_and_accepts_panel_commands() {
        use linc_core::diag::{BuildInfo, ProcessInfo};
        use linc_diag::panel::{PanelBus, PanelSource};
        use linc_panel::{
            buttons,
            sim::{SimControlState, SimPanelDriver, SimPanelHandle},
            PanelDriver,
        };
        use std::sync::atomic::{AtomicBool, Ordering};
        let dir = tempfile::tempdir().unwrap();
        let handle = SimPanelHandle::new();
        let bus = Arc::new(Mutex::new(PanelBus::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let run_stop = stop.clone();
        let run_handle = handle.clone();
        let run_bus = bus.clone();
        let poll = thread::spawn(move || {
            let mut driver = SimPanelDriver::new(run_handle, Duration::from_millis(10));
            while !run_stop.load(Ordering::Relaxed) {
                if let Ok(snapshot) = driver.poll() {
                    run_bus.lock().unwrap().publish(snapshot);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        let process = ProcessInfo {
            pid: std::process::id(),
            role: "panel".into(),
            build: BuildInfo {
                backend: "sim".into(),
                profile: "test-simulator".into(),
                ..Default::default()
            },
        };
        let source = Arc::new(PanelSource::new(
            bus,
            process,
            Some(Arc::new(SimControlState::new(handle.clone()))),
        ));
        let server = linc_diag::serve(dir.path().to_path_buf(), "panel", source).unwrap();
        let control =
            PanelPreviewControl::in_dir("test-simulator".into(), dir.path().to_path_buf());
        let wait = |check: &dyn Fn() -> bool| {
            let deadline = Instant::now() + Duration::from_secs(3);
            while !check() {
                assert!(
                    Instant::now() < deadline,
                    "timed out waiting for simulator state"
                );
                thread::sleep(Duration::from_millis(10));
            }
        };
        wait(&|| control.state().ready);
        control.send(Command::Estop(false));
        wait(&|| !handle.snapshot().estop);
        control.send(Command::Power(true));
        wait(&|| handle.snapshot().power && !handle.snapshot().estop);
        control.send(Command::Protect(true));
        wait(&|| handle.snapshot().program_protect);
        control.send(Command::Feed(1.25));
        wait(&|| (handle.snapshot().feed_override - 1.25).abs() < 0.001);
        control.send(Command::Rapid(0.50));
        wait(&|| (handle.snapshot().rapid_override - 0.50).abs() < 0.001);
        control.send(Command::Spindle(1.10));
        wait(&|| (handle.snapshot().spindle_override - 1.10).abs() < 0.001);
        control.send(Command::Mode("JOG"));
        wait(&|| handle.snapshot().mode.label() == "JOG");
        control.send(Command::CycleStart);
        wait(&|| handle.snapshot().buttons & buttons::CYCLE_START != 0);
        wait(&|| handle.snapshot().buttons & buttons::CYCLE_START == 0);
        control.send(Command::CycleStop);
        wait(&|| handle.snapshot().buttons & buttons::CYCLE_STOP != 0);
        wait(&|| handle.snapshot().buttons & buttons::CYCLE_STOP == 0);
        control.set_held(vec!["jog_xp"], control.state().connection_generation);
        wait(&|| handle.snapshot().buttons & buttons::JOG_XP != 0);
        // A stopped UI stops refreshing; the service must release the key.
        wait(&|| handle.snapshot().buttons & buttons::JOG_XP == 0);
        // Simulate the UI refreshing an old hold while the panel reconnects.
        // Even if an offline frame is never drawn, that hold must not be
        // reasserted on the next connection.
        let old_generation = control.state().connection_generation;
        control.set_held(vec!["jog_xp"], old_generation);
        wait(&|| handle.snapshot().buttons & buttons::JOG_XP != 0);
        handle.set(|state| state.error = true);
        wait(&|| !control.state().ready);
        handle.set(|state| state.error = false);
        wait(&|| {
            let state = control.state();
            state.ready && state.connection_generation != old_generation
        });
        wait(&|| handle.snapshot().buttons & buttons::JOG_XP == 0);
        let deadline = Instant::now() + Duration::from_millis(500);
        while Instant::now() < deadline {
            control.set_held(vec!["jog_xp"], old_generation);
            thread::sleep(Duration::from_millis(20));
            assert_eq!(handle.snapshot().buttons & buttons::JOG_XP, 0);
        }
        control.set_held(Vec::new(), control.state().connection_generation);
        // Exercise each distinct mill and lathe matrix key through the same
        // socket used by the embedded UI, including its release path.
        let mut names = std::collections::BTreeSet::new();
        for (lathe, second) in [(false, false), (true, true)] {
            for (_, keys) in super::super::panel_preview::panel_key_groups(lathe, second) {
                for (_, name) in keys {
                    names.insert(*name);
                }
            }
        }
        for name in names {
            let mask = linc_panel::sim::button_mask(name).unwrap();
            control.set_held(vec![name], control.state().connection_generation);
            wait(&|| handle.snapshot().buttons & mask != 0);
            control.set_held(Vec::new(), control.state().connection_generation);
            wait(&|| handle.snapshot().buttons & mask == 0);
        }
        control.send(Command::Estop(true));
        wait(&|| handle.snapshot().estop);
        drop(control);
        drop(server);
        stop.store(true, Ordering::Relaxed);
        poll.join().unwrap();
    }
}
