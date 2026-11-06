//! Simulation jog rate selection, before connecting a shared HAL jog speed.
//!
//! The selector is changed only after jog outputs have been inactive long
//! enough, and motion remains inhibited until the new selector has settled.
//! This module only computes state; it does not create pins or command motion.

use std::time::{Duration, Instant};

use crate::buttons;
use linc_core::model::{PanelMode, PanelModeValidity};

const LINEAR: u64 = buttons::JOG_XP | buttons::JOG_XN | buttons::JOG_YP
    | buttons::JOG_YN | buttons::JOG_ZP | buttons::JOG_ZN;
const ANGULAR: u64 = buttons::JOG_AP | buttons::JOG_AN | buttons::JOG_BP
    | buttons::JOG_BN | buttons::JOG_CP | buttons::JOG_CN;
const JOG: u64 = LINEAR | ANGULAR;
const DIRECTIONS: [(u64, u64); 6] = [
    (buttons::JOG_XP, buttons::JOG_XN), (buttons::JOG_YP, buttons::JOG_YN),
    (buttons::JOG_ZP, buttons::JOG_ZN), (buttons::JOG_AP, buttons::JOG_AN),
    (buttons::JOG_BP, buttons::JOG_BN), (buttons::JOG_CP, buttons::JOG_CN),
];
/// More than several servo and panel cycles on the shipped simulator; actual
/// HAL timing still requires verification in the isolated machine test.
pub const RATE_SETTLE: Duration = Duration::from_millis(100);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JogArbitration {
    pub buttons: u64,
    pub angular_selected: bool,
    pub waiting: bool,
    pub mixed_interlock: bool,
}

#[derive(Debug, Default)]
pub struct JogRateArbiter {
    angular_selected: bool,
    last_energized: Option<Instant>,
    last_directions: [(u64, Option<Instant>); 6],
    selector_changed: Option<Instant>,
    mixed_interlock: bool,
}

impl JogRateArbiter {
    /// Suppress jogging while newly published speed/step pins settle.
    pub fn settings_changed(&mut self, now: Instant) {
        self.selector_changed = Some(now);
    }
    /// Compute the selector and permitted buttons for one simulation cycle.
    /// `now` must be monotonic. A backwards timestamp inhibits selection.
    pub fn update(&mut self, buttons: u64, mode: &PanelModeValidity, now: Instant) -> JogArbitration {
        let requested = buttons & JOG;
        if requested == 0 {
            self.mixed_interlock = false;
        } else if (requested & LINEAR != 0 && requested & ANGULAR != 0)
            || DIRECTIONS.iter().any(|(plus, minus)| requested & plus != 0 && requested & minus != 0)
        {
            self.mixed_interlock = true;
        }
        let enabled = matches!(mode, PanelModeValidity::Valid(PanelMode::Jog | PanelMode::Inc));
        let mut allowed = 0;
        let mut waiting = false;
        if requested != 0 && enabled && !self.mixed_interlock {
            let wants_angular = requested & ANGULAR != 0;
            if wants_angular != self.angular_selected {
                let quiet = self.last_energized.is_none_or(|at| {
                    now.checked_duration_since(at).is_some_and(|elapsed| elapsed >= RATE_SETTLE)
                });
                if quiet {
                    self.angular_selected = wants_angular;
                    self.selector_changed = Some(now);
                }
                waiting = true;
            } else if DIRECTIONS.iter().enumerate().any(|(index, (plus, minus))| {
                let (previous, at) = self.last_directions[index];
                ((requested & plus != 0 && previous == *minus)
                    || (requested & minus != 0 && previous == *plus))
                    && at.is_some_and(|at| now.checked_duration_since(at)
                        .is_none_or(|elapsed| elapsed < RATE_SETTLE))
            }) {
                // halui processes direction changes as separate start/stop
                // edges. Let it observe the old key released before the new
                // one is asserted; otherwise the old stop can cancel the jog.
                waiting = true;
            } else if self.selector_changed.is_some_and(|at| {
                now.checked_duration_since(at).is_none_or(|elapsed| elapsed < RATE_SETTLE)
            }) {
                waiting = true;
            } else {
                allowed = requested;
                self.last_energized = Some(now);
                for (index, (plus, minus)) in DIRECTIONS.iter().enumerate() {
                    let direction = requested & (plus | minus);
                    if direction != 0 {
                        self.last_directions[index] = (direction, Some(now));
                    }
                }
            }
        }
        JogArbitration {
            buttons: (buttons & !JOG) | allowed,
            angular_selected: self.angular_selected,
            waiting,
            mixed_interlock: self.mixed_interlock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const JOG_MODE: PanelModeValidity = PanelModeValidity::Valid(PanelMode::Jog);

    #[test]
    fn changing_rate_waits_before_and_after_selecting_it() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t).buttons, buttons::JOG_XP);
        let a = arbiter.update(buttons::JOG_AP, &JOG_MODE, t + Duration::from_millis(20));
        assert_eq!(a.buttons, 0);
        assert!(!a.angular_selected && a.waiting);
        let a = arbiter.update(buttons::JOG_AP, &JOG_MODE, t + Duration::from_millis(100));
        assert!(a.angular_selected && a.waiting);
        assert_eq!(a.buttons, 0);
        assert_eq!(arbiter.update(buttons::JOG_AP, &JOG_MODE, t + Duration::from_millis(199)).buttons, 0);
        assert_eq!(arbiter.update(buttons::JOG_AP, &JOG_MODE, t + Duration::from_millis(200)).buttons, buttons::JOG_AP);
        let x = arbiter.update(buttons::JOG_XP, &JOG_MODE, t + Duration::from_millis(201));
        assert!(x.angular_selected && x.waiting);
        assert_eq!(x.buttons, 0);
        let x = arbiter.update(buttons::JOG_XP, &JOG_MODE, t + Duration::from_millis(300));
        assert!(!x.angular_selected && x.waiting);
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t + Duration::from_millis(400)).buttons, buttons::JOG_XP);
    }

    #[test]
    fn mixed_keys_latch_until_every_axis_is_released() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        let mixed = arbiter.update(buttons::JOG_XP | buttons::JOG_AP | buttons::COOLANT, &JOG_MODE, t);
        assert_eq!(mixed.buttons, buttons::COOLANT);
        assert!(mixed.mixed_interlock);
        let one = arbiter.update(buttons::JOG_XP, &JOG_MODE, t + Duration::from_secs(1));
        assert_eq!(one.buttons, 0);
        assert!(one.mixed_interlock);
        assert!(!arbiter.update(0, &JOG_MODE, t + Duration::from_secs(2)).mixed_interlock);
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t + Duration::from_secs(3)).buttons, buttons::JOG_XP);
    }

    #[test]
    fn non_jog_mode_and_backward_time_fail_closed() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        let auto = PanelModeValidity::Valid(PanelMode::Auto);
        assert_eq!(arbiter.update(buttons::JOG_XP | buttons::COOLANT, &auto, t).buttons, buttons::COOLANT);
        arbiter.update(buttons::JOG_XP, &JOG_MODE, t);
        let old = arbiter.update(buttons::JOG_AP, &JOG_MODE, t - Duration::from_millis(1));
        assert_eq!(old.buttons, 0);
        assert!(!old.angular_selected);
    }

    #[test]
    fn changed_settings_inhibit_motion_until_settled() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        arbiter.settings_changed(t);
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t).buttons, 0);
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t + RATE_SETTLE).buttons, buttons::JOG_XP);
    }

    #[test]
    fn reversal_waits_for_the_old_direction_to_be_seen_released() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        assert_eq!(arbiter.update(buttons::JOG_XP, &JOG_MODE, t).buttons, buttons::JOG_XP);
        let reverse = arbiter.update(buttons::JOG_XN, &JOG_MODE, t + Duration::from_millis(20));
        assert_eq!(reverse.buttons, 0);
        assert!(reverse.waiting);
        assert_eq!(arbiter.update(buttons::JOG_XN, &JOG_MODE, t + RATE_SETTLE).buttons, buttons::JOG_XN);
    }

    #[test]
    fn reversal_remembers_each_axis_while_another_key_stays_down() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        arbiter.update(buttons::JOG_XP | buttons::JOG_YP, &JOG_MODE, t);
        arbiter.update(buttons::JOG_YP, &JOG_MODE, t + Duration::from_millis(10));
        let reversed = buttons::JOG_XN | buttons::JOG_YP;
        assert_eq!(arbiter.update(reversed, &JOG_MODE, t + Duration::from_millis(20)).buttons, 0);
        assert_eq!(arbiter.update(reversed, &JOG_MODE, t + RATE_SETTLE).buttons, reversed);
    }

    #[test]
    fn opposite_directions_latch_until_all_jog_keys_are_released() {
        let t = Instant::now();
        let mut arbiter = JogRateArbiter::default();
        let both = arbiter.update(buttons::JOG_AP | buttons::JOG_AN, &JOG_MODE, t);
        assert_eq!(both.buttons, 0);
        assert!(both.mixed_interlock);
        assert_eq!(arbiter.update(buttons::JOG_AP, &JOG_MODE, t + RATE_SETTLE).buttons, 0);
        assert!(!arbiter.update(0, &JOG_MODE, t + RATE_SETTLE).mixed_interlock);
    }
}
