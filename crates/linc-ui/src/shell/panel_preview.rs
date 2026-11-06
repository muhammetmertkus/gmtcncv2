//! Opt-in embedded simulation panel, backed by the separate panel service.
//!
//! Controls are enabled only while the simulator service reports a ready state.
use super::*;

impl LincApp {
    pub(super) fn draw_panel_preview(&self, ui: &mut egui::Ui) {
        let panel = &self.snapshot.panel;
        ui.heading(RichText::new("SIM PANEL").color(theme::FG));
        ui.label(RichText::new("SIMULATION PREVIEW").color(theme::WARNING));
        if let Some(control) = &self.panel_preview_control {
            let state = control.state();
            let ready = state.ready
                && state
                    .checked_at
                    .is_some_and(|at| at.elapsed() < std::time::Duration::from_millis(750));
            let label = if state.estop {
                "RELEASE SIM E-STOP"
            } else {
                "ACTIVATE SIM E-STOP"
            };
            let button = egui::Button::new(
                RichText::new(label)
                    .color(if ready { theme::SURFACE } else { theme::FG })
                    .strong(),
            )
            .fill(if ready {
                theme::ERROR
            } else {
                theme::PANEL_RAISED
            })
            .min_size(Vec2::new(ui.available_width(), theme::MIN_TARGET));
            if ui.add_enabled(ready, button).clicked() {
                control.send(super::panel_preview_control::Command::Estop(!state.estop));
            }
            if !ready {
                ui.label(RichText::new("SIM CONTROL OFFLINE").color(theme::WARNING));
            }
        }
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("embedded-panel-preview-scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().interact_size.y = 0.0;
                ui.set_min_width(ui.available_width());
                if let Some(control) = &self.panel_preview_control {
                    let state = control.state();
                    let ready = state.ready
                        && state
                            .checked_at
                            .is_some_and(|at| at.elapsed() < std::time::Duration::from_millis(750));
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add_enabled(
                                ready,
                                egui::Button::new("POWER ON")
                                    .min_size(Vec2::splat(theme::MIN_TARGET)),
                            )
                            .clicked()
                        {
                            control.send(super::panel_preview_control::Command::Power(true));
                        }
                        if ui
                            .add_enabled(
                                ready,
                                egui::Button::new("POWER OFF")
                                    .min_size(Vec2::splat(theme::MIN_TARGET)),
                            )
                            .clicked()
                        {
                            control.send(super::panel_preview_control::Command::Power(false));
                        }
                        if ui
                            .add_enabled(
                                ready,
                                egui::Button::new("PROTECT")
                                    .min_size(Vec2::splat(theme::MIN_TARGET)),
                            )
                            .clicked()
                        {
                            control.send(super::panel_preview_control::Command::Protect(
                                !state.protect,
                            ));
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        for (label, led, command) in [
                            (
                                "CYCLE START",
                                "led_cycle_on",
                                super::panel_preview_control::Command::CycleStart,
                            ),
                            (
                                "CYCLE STOP",
                                "led_cycle_off",
                                super::panel_preview_control::Command::CycleStop,
                            ),
                        ] {
                            let lit = state.active_leds.iter().any(|name| name == led);
                            let text =
                                RichText::new(format!("{} {label}", if lit { "●" } else { "○" }))
                                    .color(if !lit {
                                        theme::FG
                                    } else if led == "led_cycle_off" {
                                        theme::ERROR
                                    } else {
                                        theme::SUCCESS
                                    });
                            if ui
                                .add_enabled(
                                    ready,
                                    egui::Button::new(text)
                                        .min_size(Vec2::splat(theme::MIN_TARGET)),
                                )
                                .clicked()
                            {
                                control.send(command);
                            }
                        }
                    });
                    ui.label(RichText::new("MODE").color(theme::MUTED));
                    ui.horizontal_wrapped(|ui| {
                        for mode in [
                            "AUTO",
                            "EDIT",
                            "MDI",
                            "JOG",
                            "INC",
                            "MPG",
                            "HOME",
                            "PROG.HOME",
                        ] {
                            if ui
                                .add_enabled(
                                    ready,
                                    egui::Button::new(mode)
                                        .selected(panel.mode.label() == mode)
                                        .min_size(Vec2::splat(theme::MIN_TARGET)),
                                )
                                .clicked()
                            {
                                control.send(super::panel_preview_control::Command::Mode(mode));
                            }
                        }
                    });
                    ui.separator();
                    ui.label(RichText::new("AXIS / ALARM").strong().color(theme::FG));
                    ui.horizontal_wrapped(|ui| {
                        let common = [
                            ("X", "led_x"),
                            ("Y", "led_y"),
                            ("Z", "led_z"),
                            ("A", "led_a"),
                            ("B", "led_b"),
                            ("C", "led_c"),
                            ("RUN", "led_run"),
                            ("READY", "led_ready"),
                        ];
                        let mill = [
                            ("MC", "led_mc_alarm"),
                            ("OIL", "led_oil_alarm"),
                            ("GEAR", "led_gear_alarm"),
                            ("AIR", "led_airblow_alarm"),
                        ];
                        let lathe = [
                            ("NC", "led_nc"),
                            ("PLC", "led_plc"),
                            ("MMC", "led_mmc"),
                            ("EMG", "led_emg"),
                        ];
                        let alarms = if self.profile.variant == MachineVariant::Lathe {
                            &lathe
                        } else {
                            &mill
                        };
                        for (caption, led) in common.iter().chain(alarms.iter()) {
                            let estop_lamp = if self.profile.variant == MachineVariant::Lathe {
                                "led_emg"
                            } else {
                                "led_mc_alarm"
                            };
                            let active = state.active_leds.iter().any(|name| name == *led)
                                || (*led == estop_lamp && state.estop);
                            let color = if !active {
                                theme::MUTED
                            } else if *led == "led_run" {
                                theme::WARNING
                            } else if alarms.iter().any(|(_, alarm)| alarm == led) {
                                theme::ERROR
                            } else {
                                theme::SUCCESS
                            };
                            ui.add(
                                egui::Label::new(
                                    RichText::new(format!("● {caption}")).color(color),
                                )
                                .wrap_mode(egui::TextWrapMode::Extend),
                            );
                        }
                    });
                    let mut key_rects = Vec::new();
                    let lathe = self.profile.variant == MachineVariant::Lathe;
                    let sub_spindle = self
                        .profile
                        .config
                        .lathe
                        .as_ref()
                        .and_then(|config| config.sub_spindle)
                        .unwrap_or(false);
                    for (group, keys) in panel_key_groups(lathe, sub_spindle) {
                        ui.label(RichText::new(group).strong().color(theme::FG));
                        let gap = 4.0;
                        let width =
                            ((ui.available_width() - gap * 2.0) / 3.0).max(theme::MIN_TARGET);
                        for row in keys.chunks(3) {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = gap;
                                for (label, name) in row {
                                    let wired = !state
                                        .disconnected_buttons
                                        .iter()
                                        .any(|unwired| unwired == name);
                                    let led_name = if *name == "lube" {
                                        "led_lub".to_string()
                                    } else {
                                        format!("led_{name}")
                                    };
                                    let active =
                                        state.active_leds.iter().any(|led| led == &led_name);
                                    let mask = linc_core::panel_buttons::BUTTON_PINS
                                        .iter()
                                        .find(|(entry, _)| entry == name)
                                        .map(|(_, mask)| *mask);
                                    let icon = mask.and_then(|mask| {
                                        linc_view::panel_icons::for_key(
                                            mask,
                                            icon_arrow(label, lathe),
                                        )
                                    });
                                    let button = egui::Button::new("").fill(theme::SURFACE);
                                    let response = ui
                                        .add_enabled_ui(ready && wired, |ui| {
                                            ui.add_sized([width, 64.0], button)
                                        })
                                        .inner;
                                    let color = if ready && wired {
                                        theme::FG
                                    } else {
                                        theme::MUTED
                                    };
                                    let painter = ui.painter();
                                    if let Some(icon) = icon {
                                        let icon_rect = egui::Rect::from_center_size(
                                            egui::pos2(
                                                response.rect.center().x,
                                                response.rect.top() + 23.0,
                                            ),
                                            Vec2::new((width - 8.0).max(24.0), 27.0),
                                        );
                                        linc_view::panel_icons::paint(
                                            painter, icon, icon_rect, color,
                                        );
                                    }
                                    painter.text(
                                        egui::pos2(
                                            response.rect.center().x,
                                            response.rect.bottom() - 9.0,
                                        ),
                                        Align2::CENTER_CENTER,
                                        *label,
                                        FontId::proportional(theme::font_size(8.0)),
                                        color,
                                    );
                                    painter.circle_filled(
                                        response.rect.left_top() + Vec2::splat(7.0),
                                        3.0,
                                        if active {
                                            theme::SUCCESS
                                        } else {
                                            theme::BORDER
                                        },
                                    );
                                    if ready && wired {
                                        let rect = response.rect.intersect(ui.clip_rect());
                                        if rect.is_positive() {
                                            key_rects.push((*name, rect));
                                        }
                                    }
                                }
                            });
                        }
                    }
                    if state.probe_enabled {
                        let response = ui.add_enabled(
                            ready,
                            egui::Button::new("PROBE CONTACT")
                                .min_size(Vec2::splat(theme::MIN_TARGET)),
                        );
                        if ready {
                            let rect = response.rect.intersect(ui.clip_rect());
                            if rect.is_positive() {
                                key_rects.push(("probe_contact", rect));
                            }
                        }
                    }
                    let id = egui::Id::unique("embedded-panel-held-inputs");
                    let mut holds = ui
                        .ctx()
                        .data_mut(|data| data.get_temp::<EmbeddedHolds>(id).unwrap_or_default());
                    holds.update_connection(state.connection_generation);
                    let (events, focused) = ui.input(|input| (input.events.clone(), input.focused));
                    holds.process(
                        &events,
                        &key_rects,
                        focused && ready && !self.overlay_open(),
                    );
                    let keys = holds.keys();
                    control.set_held(keys.clone(), state.connection_generation);
                    ui.ctx().data_mut(|data| data.insert_temp(id, holds));
                    if !keys.is_empty() {
                        ui.ctx()
                            .request_repaint_after(std::time::Duration::from_millis(50));
                    }
                    // Values are percentages in the panel UI and ratios on the wire.
                    for (label, current, range, make_command) in [
                        (
                            "FEED %",
                            state.feed,
                            0.0..=150.0,
                            super::panel_preview_control::Command::Feed as fn(f64) -> _,
                        ),
                        (
                            "RAPID %",
                            state.rapid,
                            0.0..=100.0,
                            super::panel_preview_control::Command::Rapid as fn(f64) -> _,
                        ),
                        (
                            "SPINDLE %",
                            state.spindle,
                            50.0..=120.0,
                            super::panel_preview_control::Command::Spindle as fn(f64) -> _,
                        ),
                    ] {
                        let id = egui::Id::unique(("embedded-override-pending", label));
                        let mut pending = ui
                            .ctx()
                            .data_mut(|data| data.get_temp::<PendingOverride>(id));
                        let mut percent = pending
                            .as_ref()
                            .and_then(|value| value.display(current, ready))
                            .unwrap_or(current)
                            * 100.0;
                        let response = ui.add_enabled(
                            ready,
                            egui::Slider::new(&mut percent, range).text(label).integer(),
                        );
                        if response.changed() {
                            let ratio = percent / 100.0;
                            control.send(make_command(ratio));
                            pending = Some(PendingOverride {
                                value: ratio,
                                sent_at: std::time::Instant::now(),
                            });
                        } else if pending
                            .as_ref()
                            .is_some_and(|value| value.display(current, ready).is_none())
                        {
                            pending = None;
                        }
                        ui.ctx().data_mut(|data| {
                            if let Some(value) = pending {
                                data.insert_temp(id, value);
                            } else {
                                data.remove::<PendingOverride>(id);
                            }
                        });
                    }
                }
                ui.separator();
                let status = if panel.connected {
                    "CONNECTED"
                } else {
                    "OFFLINE"
                };
                ui.label(RichText::new(status).strong().color(if panel.connected {
                    theme::SUCCESS
                } else {
                    theme::ERROR
                }));
                ui.add_space(8.0);
                panel_value(ui, "MODE", panel.mode.label());
                panel_value(
                    ui,
                    "E-STOP",
                    if panel.estop { "ACTIVE" } else { "RELEASED" },
                );
                panel_value(
                    ui,
                    "POWER",
                    match panel.power_on {
                        Some(true) => "ON",
                        Some(false) => "OFF",
                        None => "UNKNOWN",
                    },
                );
                panel_value(
                    ui,
                    "PROGRAM PROTECT",
                    if !panel.program_protect_known {
                        "UNKNOWN"
                    } else if panel.program_protect {
                        "ON"
                    } else {
                        "OFF"
                    },
                );
                ui.separator();
                panel_value(
                    ui,
                    "FEED OVERRIDE",
                    &format!("{:.0}%", panel.feed_override * 100.0),
                );
                panel_value(
                    ui,
                    "RAPID OVERRIDE",
                    &format!("{:.0}%", panel.rapid_override * 100.0),
                );
                panel_value(
                    ui,
                    "SPINDLE OVERRIDE",
                    &format!("{:.0}%", panel.spindle_override * 100.0),
                );
                ui.separator();
                let lathe = self.profile.variant == MachineVariant::Lathe;
                panel_value(
                    ui,
                    if lathe { "TOOL." } else { "MAG TOOL" },
                    &panel.sseg_tool.to_string(),
                );
                panel_value(
                    ui,
                    if lathe { "TOOL COMP." } else { "SPD TOOL" },
                    &panel.sseg_next_tool.to_string(),
                );
                let status = if (54..=59).contains(&panel.sseg_status) {
                    panel.sseg_status.to_string()
                } else {
                    "--".to_owned()
                };
                panel_value(ui, if lathe { "STATUS." } else { "STATUS" }, &status);
                ui.add_space(12.0);
                ui.label(RichText::new("Simulation panel controls").color(theme::MUTED));
            });
    }
}

/// Preserve a recently requested slider value until the service echoes it or
/// the short acknowledgement window expires. Never retain it offline.
#[derive(Clone)]
struct PendingOverride {
    value: f64,
    sent_at: std::time::Instant,
}
impl PendingOverride {
    fn display(&self, observed: f64, ready: bool) -> Option<f64> {
        if !ready
            || (observed - self.value).abs() < 0.005
            || self.sent_at.elapsed() >= std::time::Duration::from_millis(750)
        {
            None
        } else {
            Some(self.value)
        }
    }
}

fn panel_value(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).color(theme::MUTED));
        ui.label(RichText::new(value).strong().color(theme::FG));
    });
    ui.add_space(6.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luminance(color: Color32) -> f64 {
        let channel = |v: u8| {
            let v = f64::from(v) / 255.0;
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
    }

    #[test]
    fn pending_override_survives_delayed_echo_but_not_disconnect() {
        let pending = PendingOverride {
            value: 1.25,
            sent_at: std::time::Instant::now(),
        };
        assert_eq!(pending.display(1.0, true), Some(1.25));
        assert_eq!(pending.display(1.25, true), None);
        assert_eq!(pending.display(1.0, false), None);
        let expired = PendingOverride {
            value: 1.25,
            sent_at: std::time::Instant::now() - std::time::Duration::from_secs(1),
        };
        assert_eq!(expired.display(1.0, true), None);
    }

    #[test]
    fn every_embedded_matrix_key_maps_to_a_panel_button() {
        for (lathe, sub_spindle) in [(false, false), (true, false), (true, true)] {
            let groups = panel_key_groups(lathe, sub_spindle);
            let mut seen = std::collections::HashSet::new();
            for (_, keys) in groups {
                for (_, name) in keys {
                    assert!(
                        linc_panel::sim::button_mask(name).is_some(),
                        "unknown simulator button {name}"
                    );
                    assert!(seen.insert(*name), "duplicate button {name}");
                }
            }
            let expected = if lathe {
                if sub_spindle {
                    42
                } else {
                    39
                }
            } else {
                42
            };
            assert_eq!(seen.len(), expected);
        }
    }

    #[test]
    fn mill_jog_keys_use_the_shared_panel_pictograms() {
        for (label, name) in MILL_JOG {
            if *name == "safe_door" || *name == "rapid" || name.starts_with("jog_") {
                let mask = linc_core::panel_buttons::BUTTON_PINS
                    .iter()
                    .find(|(candidate, _)| candidate == name)
                    .unwrap()
                    .1;
                assert!(
                    linc_view::panel_icons::for_key(mask, icon_arrow(label, false)).is_some(),
                    "missing icon for {name}"
                );
            }
        }
    }

    #[test]
    fn holds_follow_their_own_touch_and_clear_on_focus_loss() {
        let mut holds = EmbeddedHolds::default();
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(50.0, 50.0));
        let event = |id, phase| egui::Event::Touch {
            device_id: egui::TouchDeviceId(1),
            id: egui::TouchId(id),
            phase,
            pos: egui::pos2(10.0, 10.0),
            force: None,
        };
        holds.process(
            &[
                event(1, egui::TouchPhase::Start),
                event(2, egui::TouchPhase::Start),
            ],
            &[("jog_xp", rect)],
            true,
        );
        assert_eq!(holds.keys(), vec!["jog_xp"]);
        holds.process(
            &[event(2, egui::TouchPhase::End)],
            &[("jog_xp", rect)],
            true,
        );
        assert_eq!(holds.keys(), vec!["jog_xp"]);
        holds.process(&[], &[], false);
        assert!(holds.keys().is_empty());
    }

    #[test]
    fn a_reconnected_panel_requires_a_new_press() {
        let mut holds = EmbeddedHolds::default();
        holds.update_connection(1);
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(50.0, 50.0));
        let press = egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        };
        holds.process(&[press], &[("jog_xp", rect)], true);
        assert_eq!(holds.keys(), vec!["jog_xp"]);
        holds.update_connection(2);
        holds.process(&[], &[("jog_xp", rect)], true);
        assert!(holds.keys().is_empty());
    }

    #[test]
    fn preview_text_has_at_least_4_5_to_1_contrast_on_panel() {
        let background = luminance(theme::PANEL);
        for (name, color) in [
            ("value", theme::FG),
            ("label", theme::MUTED),
            ("warning", theme::WARNING),
            ("offline", theme::ERROR),
            ("connected", theme::SUCCESS),
        ] {
            let text = luminance(color);
            let ratio = (text.max(background) + 0.05) / (text.min(background) + 0.05);
            assert!(ratio >= 4.5, "{name} contrast is {ratio:.2}:1");
        }
    }
}

/// Tracks the input source that originally pressed each key. A second finger
/// releasing must not cancel the first finger's jog.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EmbeddedSource {
    Mouse,
    Touch(egui::TouchId),
}

#[derive(Clone, Default)]
struct EmbeddedHolds {
    sources: Vec<(EmbeddedSource, &'static str)>,
    connection_generation: u64,
}

impl EmbeddedHolds {
    fn update_connection(&mut self, generation: u64) {
        if self.connection_generation != generation {
            self.sources.clear();
            self.connection_generation = generation;
        }
    }
    fn process(
        &mut self,
        events: &[egui::Event],
        rects: &[(&'static str, egui::Rect)],
        focused: bool,
    ) {
        if !focused {
            self.sources.clear();
            return;
        }
        self.sources
            .retain(|(_, key)| rects.iter().any(|(name, _)| name == key));
        let hit = |pos: egui::Pos2| {
            rects
                .iter()
                .find(|(_, rect)| rect.contains(pos))
                .map(|(name, _)| *name)
        };
        for event in events {
            match *event {
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    ..
                } => {
                    if pressed {
                        if let Some(name) = hit(pos) {
                            self.sources
                                .retain(|(source, _)| *source != EmbeddedSource::Mouse);
                            self.sources.push((EmbeddedSource::Mouse, name));
                        }
                    } else {
                        self.sources
                            .retain(|(source, _)| *source != EmbeddedSource::Mouse);
                    }
                }
                egui::Event::PointerGone => self
                    .sources
                    .retain(|(source, _)| *source != EmbeddedSource::Mouse),
                egui::Event::Touch { id, phase, pos, .. } => {
                    let source = EmbeddedSource::Touch(id);
                    match phase {
                        egui::TouchPhase::Start => {
                            if let Some(name) = hit(pos) {
                                self.sources.retain(|(held, _)| *held != source);
                                self.sources.push((source, name));
                            }
                        }
                        egui::TouchPhase::End | egui::TouchPhase::Cancel => {
                            self.sources.retain(|(held, _)| *held != source)
                        }
                        egui::TouchPhase::Move => {}
                    }
                }
                _ => {}
            }
        }
    }
    fn keys(&self) -> Vec<&'static str> {
        let mut keys = Vec::new();
        for (_, name) in &self.sources {
            if !keys.contains(name) {
                keys.push(*name);
            }
        }
        keys
    }
}

type EmbeddedKey = (&'static str, &'static str);
const MILL_JOG: &[EmbeddedKey] = &[
    ("B+", "jog_bp"),
    ("SAFE DOOR", "safe_door"),
    ("B−", "jog_bn"),
    ("A+", "jog_ap"),
    ("Z+", "jog_zp"),
    ("Y+", "jog_yp"),
    ("X−", "jog_xn"),
    ("RAPID", "rapid"),
    ("X+", "jog_xp"),
    ("Y−", "jog_yn"),
    ("Z−", "jog_zn"),
    ("A−", "jog_an"),
];
const MILL_MIDDLE: &[EmbeddedKey] = &[
    ("MPG SIM", "mpg_sim"),
    ("S.B.K.", "sbk"),
    ("B.D.T.", "bdt"),
    ("M.LK", "mlk"),
    ("OPT", "opt"),
    ("D.R.N.", "drn"),
    ("WORKLAMP", "worklamp"),
    ("AIR BLOW", "air_blow"),
    ("PT.LA", "pt_la"),
    ("COOLANT", "coolant"),
    ("SPD.CTS", "spd_cts"),
    ("FLUSH", "flush"),
    ("LUB", "lube"),
    ("CORRIDOR", "corridor"),
    ("K1", "k1"),
];
const MILL_RIGHT: &[EmbeddedKey] = &[
    ("SPD.ORT", "spd_ori"),
    ("MAG CW", "mag_cw"),
    ("MAG CCW", "mag_ccw"),
    ("SPD.CW", "spd_cw"),
    ("SPD.SP", "spd_sp"),
    ("SPD.CCW", "spd_ccw"),
    ("CHIP CW", "chip_cw"),
    ("CHIP SP", "chip_sp"),
    ("CHIP CCW", "chip_ccw"),
    ("CHUCK", "chuck"),
    ("POT DN", "pot_dn"),
    ("ARM MAN", "arm_man"),
    ("K2", "k2"),
    ("K3", "k3"),
    ("K4", "k4"),
];
const LATHE_JOG: &[EmbeddedKey] = &[
    ("C−", "jog_cn"),
    ("C/S", "c_s"),
    ("C+", "jog_cp"),
    ("Y+", "jog_yp"),
    ("X+", "jog_xp"),
    ("A+", "jog_ap"),
    ("Z−", "jog_zn"),
    ("RAPID", "rapid"),
    ("Z+", "jog_zp"),
    ("A−", "jog_an"),
    ("X−", "jog_xn"),
    ("Y−", "jog_yn"),
];
const LATHE_MIDDLE: &[EmbeddedKey] = &[
    ("MPG SIM", "mpg_sim"),
    ("SAFE DOOR", "safe_door"),
    ("CORRIDOR", "corridor"),
    ("S.B.K.", "sbk"),
    ("B.D.T.", "bdt"),
    ("OPT", "opt"),
    ("M.LK", "mlk"),
    ("MST", "mst"),
    ("D.R.N.", "drn"),
    ("B+", "jog_bp"),
    ("K1", "k1"),
    ("K3", "k3"),
    ("B−", "jog_bn"),
    ("K2", "k2"),
    ("K4", "k4"),
];
const LATHE_SECOND: &[EmbeddedKey] = &[
    ("SPD.2.CW", "spd2_cw"),
    ("SPD.2.SP", "spd2_sp"),
    ("SPD.2.CCW", "spd2_ccw"),
];
const LATHE_RIGHT: &[EmbeddedKey] = &[
    ("SPD.CW", "spd_cw"),
    ("SPD.SP", "spd_sp"),
    ("SPD.CCW", "spd_ccw"),
    ("SPD. JOG", "spd_jog"),
    ("FEED BRAKE", "feed_brake"),
    ("LUB", "lube"),
    ("CHUCK", "chuck"),
    ("TAIL", "tail"),
    ("CHIP", "chip"),
    ("COOLANT", "coolant"),
    ("TOOL CHG.", "tool_chg"),
    ("WORKLAMP", "worklamp"),
];
pub(super) fn panel_key_groups(
    lathe: bool,
    sub_spindle: bool,
) -> Vec<(&'static str, &'static [EmbeddedKey])> {
    if lathe {
        let mut groups = vec![("JOG", LATHE_JOG), ("FUNCTIONS", LATHE_MIDDLE)];
        if sub_spindle {
            groups.push(("SECOND SPINDLE", LATHE_SECOND));
        }
        groups.push(("SPINDLE / MACHINE", LATHE_RIGHT));
        groups
    } else {
        vec![
            ("JOG", MILL_JOG),
            ("FUNCTIONS", MILL_MIDDLE),
            ("SPINDLE / MACHINE", MILL_RIGHT),
        ]
    }
}

fn icon_arrow(label: &str, lathe: bool) -> Option<&'static str> {
    match label {
        "X−" => Some(if lathe { "v" } else { "<" }),
        "X+" => Some(if lathe { "^" } else { ">" }),
        "Y−" => Some("v"),
        "Y+" => Some("^"),
        "Z−" => Some(if lathe { "<" } else { "v" }),
        "Z+" => Some(if lathe { ">" } else { "^" }),
        "A−" => Some("v"),
        "A+" => Some("^"),
        "B−" => Some(if lathe { ">" } else { "v" }),
        "B+" => Some(if lathe { "<" } else { "^" }),
        "C−" => Some("<"),
        "C+" => Some(">"),
        _ => None,
    }
}
