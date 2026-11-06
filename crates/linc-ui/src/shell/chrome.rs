//! Fixed chrome: the top bar, the bottom bar, the messages strip, banners and
//! the icon and metric helpers they paint with.

use super::*;

#[allow(dead_code)]
pub(super) fn metric(ui: &mut egui::Ui, label: &str, value: String, size: f32) -> egui::Response {
    metric_colored(ui, label, value, size, theme::FG)
}

/// A top-bar metric whose value carries a state colour: the label stays quiet,
/// the value is what has to be read at a glance.
#[allow(dead_code)]
pub(super) fn metric_colored(
    ui: &mut egui::Ui,
    label: &str,
    value: String,
    size: f32,
    color: Color32,
) -> egui::Response {
    ui.add_space(6.0);
    ui.vertical(|ui| {
        ui.label(
            RichText::new(label)
                .size(theme::font_size(9.0))
                .color(theme::MUTED),
        );
        ui.label(RichText::new(value).monospace().size(size).color(color));
    })
    .response
}

pub(super) fn cockpit_capsule_custom<R>(
    ui: &mut egui::Ui,
    bg: Color32,
    border_stroke: egui::Stroke,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> (egui::Response, R) {
    let frame = egui::Frame::new()
        .fill(bg)
        .stroke(border_stroke)
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(8, 2));
    let mut inner = None;
    let resp = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            inner = Some(add_contents(ui));
        });
    });
    let rect = resp.response.rect;
    let interactive_resp = ui.interact(rect, resp.response.id, egui::Sense::click());
    if interactive_resp.hovered() {
        ui.painter().rect_stroke(
            rect,
            8,
            egui::Stroke::new(1.0, theme::COCKPIT_BORDER_ACTIVE),
            egui::StrokeKind::Inside,
        );
    }
    (interactive_resp, inner.unwrap())
}

pub(super) fn cockpit_chip<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> (egui::Response, R) {
    cockpit_capsule_custom(
        ui,
        theme::COCKPIT_CARD,
        egui::Stroke::new(1.0, theme::COCKPIT_BORDER),
        add_contents,
    )
}

impl LincApp {
    pub(super) fn top_bar(&mut self, ui: &mut egui::Ui) {
        // Reserve the right edge for safety badges and hardware status;
        // Telemetry capsules are allocated inside an independent bounded scroll area,
        // making collision or overlapping with safety badges physically impossible.
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            // 1. Bildirim & Alarm Merkezi [MSG 4 | ALARM 2]
            let (messages, alarms) = self.log_counts();
            let has_alarms = alarms > 0;
            let (alarm_resp, _) = cockpit_capsule_custom(
                ui,
                if has_alarms {
                    Color32::from_rgba_unmultiplied(239, 68, 68, 35)
                } else {
                    theme::COCKPIT_CARD
                },
                egui::Stroke::new(
                    1.0,
                    if has_alarms {
                        Color32::from_rgba_unmultiplied(239, 68, 68, 120)
                    } else {
                        theme::COCKPIT_BORDER
                    },
                ),
                |ui| {
                    ui.label(
                        RichText::new(format!("MSG {messages}"))
                            .monospace()
                            .size(theme::font_size(10.5))
                            .color(if messages > 0 { theme::COCKPIT_TEXT_MAIN } else { theme::COCKPIT_TEXT_DIM }),
                    );
                    ui.label(
                        RichText::new("|")
                            .size(10.0)
                            .color(theme::COCKPIT_TEXT_DIM),
                    );
                    ui.label(
                        RichText::new(format!("ALARM {alarms}"))
                            .monospace()
                            .size(theme::font_size(10.5))
                            .strong()
                            .color(if has_alarms { theme::COCKPIT_RED } else { theme::COCKPIT_TEXT_DIM }),
                    );
                },
            );
            if alarm_resp.clicked() {
                self.nav.page = Page::Diagnostics;
            }
            alarm_resp.on_hover_ui(|ui| {
                ui.label(RichText::new("SİSTEM BİLDİRİM VE ALARM MERKEZİ").strong().color(theme::COCKPIT_RED));
                for alarm in self.snapshot.alarms.iter().take(5) {
                    let colour = match alarm.severity {
                        linc_core::model::AlarmSeverity::Critical
                        | linc_core::model::AlarmSeverity::Error => theme::COCKPIT_RED,
                        linc_core::model::AlarmSeverity::Warning => theme::COCKPIT_AMBER,
                        linc_core::model::AlarmSeverity::Info => theme::COCKPIT_TEXT_MUTED,
                    };
                    let text = linc_core::i18n::translate_cnc_message(&alarm.message);
                    ui.label(RichText::new(format!("• {text}")).size(11.0).color(colour));
                }
            });

            // 2. Donanım Panel / MPG El Çarkı Bağlantı Hapı [🎛 ● PANEL]
            let online = !self.panel_offline();
            let (panel_resp, _) = cockpit_chip(ui, |ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 14.0), Sense::hover());
                ui.painter().rect_stroke(
                    icon_rect,
                    2.0,
                    egui::Stroke::new(1.0, if online { theme::COCKPIT_GREEN } else { theme::COCKPIT_RED }),
                    StrokeKind::Inside,
                );
                ui.painter().circle_filled(icon_rect.center(), 1.5, if online { theme::COCKPIT_GREEN } else { theme::COCKPIT_RED });

                let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover());
                ui.painter().circle_filled(dot_rect.center(), 3.0, if online { theme::COCKPIT_GREEN } else { theme::COCKPIT_RED });

                ui.label(RichText::new("PANEL").monospace().size(theme::font_size(10.5)).strong().color(Color32::WHITE));
            });
            panel_resp.on_hover_text(if online { "PANEL & EL ÇARKI (MPG) AKTİF" } else { "PANEL ÇEVRİMDIŞI" });

            // 3. Güvenlik Rozetleri ([LİMİT OVR], [🛑 ESTOP], vb.)
            self.status_badges(ui);

            // 4. Sol ve Orta Telemetri Kapsülleri (Kalan alanda soldan sağa dizilim)
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 6.0;

                // 1. Mod Kapsülü [PANEL AUTO • TASK MANUAL ▾]
                let panel_mode_label = format!("PANEL {}", self.snapshot.panel.mode.label());
                let task_mode_label = format!("TASK {}", self.snapshot.task.mode.label());
                let panel_valid = self.snapshot.panel.mode.is_valid();
                let (mode_resp, _) = cockpit_chip(ui, |ui| {
                    ui.label(
                        RichText::new(panel_mode_label)
                            .monospace()
                            .size(theme::font_size(10.5))
                            .strong()
                            .color(if panel_valid { theme::COCKPIT_CYAN } else { theme::COCKPIT_RED }),
                    );
                    ui.label(RichText::new("•").monospace().size(10.5).color(theme::COCKPIT_TEXT_DIM));
                    ui.label(
                        RichText::new(task_mode_label)
                            .monospace()
                            .size(theme::font_size(10.5))
                            .strong()
                            .color(theme::COCKPIT_TEXT_MAIN),
                    );
                    ui.label(RichText::new("▾").size(9.0).color(theme::COCKPIT_TEXT_DIM));
                });
                mode_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("ÇALIŞMA MODU DETAYLARI").strong().color(theme::COCKPIT_CYAN));
                    ui.label("• Panel Modu: Ön panel veya dokunmatik ekran otomatik komut kabul modundadır.");
                    ui.label("• Task Modu: LinuxCNC iç iş parçacığı yürütme durumunu gösterir.");
                });

                // 2. Program Dosya Kapsülü [📄 PRG: gmt_parca.ngc]
                let prog_label = self.program_label();
                let (prog_resp, _) = cockpit_chip(ui, |ui| {
                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(12.0, 14.0), Sense::hover());
                    ui.painter().rect_stroke(icon_rect, 1.5, egui::Stroke::new(1.0, theme::COCKPIT_BLUE), StrokeKind::Inside);
                    ui.painter().text(icon_rect.center(), Align2::CENTER_CENTER, "G", FontId::monospace(7.5), theme::COCKPIT_BLUE);

                    ui.label(RichText::new("PRG:").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(&prog_label).monospace().size(theme::font_size(11.5)).strong().color(Color32::WHITE));
                });
                prog_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("AKTİF G-KODU PROGRAMI").strong().color(theme::ACCENT));
                    ui.label(format!("Dosya: {}", prog_label));
                });

                // 3. Durum Kapsülü [● STATE: ÇALIŞIYOR / IDLE / PAUSED]
                let (state_word, state_color) = program_state_word(&self.snapshot.program);
                let is_running = state_word.eq_ignore_ascii_case("running") || state_word.eq_ignore_ascii_case("çalışıyor");
                let is_paused = state_word.eq_ignore_ascii_case("paused") || state_word.eq_ignore_ascii_case("beklemede");
                let (state_resp, _) = cockpit_chip(ui, |ui| {
                    let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                    let dot_color = if is_running {
                        theme::COCKPIT_GREEN
                    } else if is_paused {
                        theme::COCKPIT_AMBER
                    } else {
                        theme::COCKPIT_TEXT_DIM
                    };
                    ui.painter().circle_filled(dot_rect.center(), 3.5, dot_color);
                    if is_running {
                        ui.painter().circle_stroke(dot_rect.center(), 5.0, egui::Stroke::new(1.0, theme::tint(theme::COCKPIT_GREEN, 120)));
                    }
                    ui.label(RichText::new("STATE:").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(state_word).monospace().size(theme::font_size(11.5)).strong().color(state_color));
                });
                state_resp.on_hover_text("Tezgâh Durumu (IDLE / RUNNING / PAUSED)");

                // 4. Takım Kapsülü [TOOL T1 Ф10]
                let current_tool = self.snapshot.tool_change.current_tool;
                let active_tool_row = self.snapshot.tool_table.rows.iter().find(|r| r.tool == current_tool).cloned();
                let dia_str = active_tool_row
                    .as_ref()
                    .and_then(|t| t.geometry.diameter)
                    .map(|d| format!("Ф{:.1}", d))
                    .unwrap_or_else(|| "Ф--".into());
                let (tool_resp, _) = cockpit_chip(ui, |ui| {
                    ui.label(RichText::new("TOOL").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(format!("T{current_tool}")).monospace().size(theme::font_size(11.5)).strong().color(theme::COCKPIT_GREEN));
                    ui.label(RichText::new(dia_str).monospace().size(theme::font_size(10.5)).color(theme::COCKPIT_TEXT_MUTED));
                });
                tool_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("AKTİF SPINDLE TAKIMI").strong().color(theme::SUCCESS));
                    ui.label(format!("Takım Numarası: T{current_tool}"));
                    if let Some(tool) = active_tool_row.as_ref() {
                        if !tool.description.is_empty() {
                            ui.label(format!("Tanım: {}", tool.description));
                        }
                        if let Some(dia) = tool.geometry.diameter {
                            ui.label(format!("Çap (D): {:.2} mm", dia));
                        }
                        if let Some(z) = tool.geometry.values.get("Z") {
                            ui.label(format!("Boy Telafisi (Z): {:.3} mm", z));
                        } else if let Some(len) = tool.geometry.length {
                            ui.label(format!("Boy Telafisi (L): {:.3} mm", len));
                        }
                    }
                });

                // 5. Spindle Devir Kapsülü [🗘 SPINDLE S 18.000 100% ▾]
                let spindle_speed_text = self.snapshot.spindles.first()
                    .map(|spindle| format!("S {:.0}", spindle.actual_speed))
                    .unwrap_or_else(|| "S --".into());
                let (spindle_resp, _) = cockpit_chip(ui, |ui| {
                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
                    ui.painter().circle_stroke(icon_rect.center(), 4.5, egui::Stroke::new(1.2, theme::COCKPIT_AMBER));
                    ui.painter().circle_filled(icon_rect.center(), 1.5, theme::COCKPIT_AMBER);

                    ui.label(RichText::new("SPINDLE").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(&spindle_speed_text).monospace().size(theme::font_size(11.5)).strong().color(theme::COCKPIT_AMBER));

                    let (pct_rect, _) = ui.allocate_exact_size(Vec2::new(32.0, 16.0), Sense::hover());
                    ui.painter().rect_filled(pct_rect, 3, Color32::from_rgba_unmultiplied(245, 158, 11, 35));
                    ui.painter().text(pct_rect.center(), Align2::CENTER_CENTER, "100%", FontId::monospace(theme::font_size(8.5)), theme::COCKPIT_AMBER);
                    ui.label(RichText::new("▾").size(9.0).color(theme::COCKPIT_TEXT_DIM));
                });
                spindle_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("SPINDLE DEVİR DURUMU").strong().color(theme::WARNING));
                    if let Some(spindle) = self.snapshot.spindles.first() {
                        ui.label(format!("Anlık Hız: {:.0} RPM", spindle.actual_speed));
                    }
                    ui.label("Potansiyometre Devir Çarpanı: %100");
                });

                // 6. İlerleme Hızı Kapsülü [>> FEED F 1.200 100% ▾]
                let feed_text = format!("F {:.0}", self.snapshot.feed.feed);
                let (feed_resp, _) = cockpit_chip(ui, |ui| {
                    ui.label(RichText::new(">>").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_BLUE));
                    ui.label(RichText::new("FEED").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(&feed_text).monospace().size(theme::font_size(11.5)).strong().color(theme::COCKPIT_BLUE));

                    let (pct_rect, _) = ui.allocate_exact_size(Vec2::new(32.0, 16.0), Sense::hover());
                    ui.painter().rect_filled(pct_rect, 3, Color32::from_rgba_unmultiplied(59, 130, 246, 35));
                    ui.painter().text(pct_rect.center(), Align2::CENTER_CENTER, "100%", FontId::monospace(theme::font_size(8.5)), theme::COCKPIT_BLUE);
                    ui.label(RichText::new("▾").size(9.0).color(theme::COCKPIT_TEXT_DIM));
                });
                feed_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("İLERLEME HIZI (FEED)").strong().color(theme::ACCENT));
                    ui.label(format!("Efektif Hız: {:.0} mm/dk", self.snapshot.feed.feed));
                });

                // 7. İş Parçası Koordinat Kapsülü [WORK G54 ▾]
                let wcs_text = self.active_wcs_label();
                let (work_resp, _) = cockpit_chip(ui, |ui| {
                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
                    ui.painter().circle_stroke(icon_rect.center(), 4.5, egui::Stroke::new(1.1, theme::COCKPIT_CYAN));
                    ui.painter().line_segment([egui::pos2(icon_rect.center().x, icon_rect.top()), egui::pos2(icon_rect.center().x, icon_rect.bottom())], egui::Stroke::new(1.0, theme::COCKPIT_CYAN));
                    ui.painter().line_segment([egui::pos2(icon_rect.left(), icon_rect.center().y), egui::pos2(icon_rect.right(), icon_rect.center().y)], egui::Stroke::new(1.0, theme::COCKPIT_CYAN));

                    ui.label(RichText::new("WORK").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                    ui.label(RichText::new(&wcs_text).monospace().size(theme::font_size(11.5)).strong().color(theme::COCKPIT_CYAN));
                    ui.label(RichText::new("▾").size(9.0).color(theme::COCKPIT_TEXT_DIM));
                });
                work_resp.on_hover_ui(|ui| {
                    ui.label(RichText::new("İŞ PARÇASI KOORDİNAT SİSTEMİ (WCS)").strong().color(theme::ACCENT));
                    ui.label(format!("Aktif Sıfır: {wcs_text}"));
                    ui.label("Diğer ofsetler (G55-G59) için Ofsetler (F8) sekmesini açın.");
                });

                // 8. Ağ Denetim Kapsülü [NET OFF]
                if let Some((network_word, network_color)) = network_metric(&self.snapshot) {
                    let (net_resp, _) = cockpit_chip(ui, |ui| {
                        ui.label(RichText::new("NET").monospace().size(theme::font_size(10.0)).strong().color(theme::COCKPIT_TEXT_DIM));
                        ui.label(RichText::new(network_word).monospace().size(theme::font_size(11.0)).strong().color(network_color));
                    });
                    net_resp.on_hover_ui(|ui| {
                        ui.label(RichText::new("HARİCİ AĞ DENETİMİ (NET)").strong().color(theme::MUTED));
                        ui.label(format!("Durum: {network_word}"));
                        ui.label("LinuxCNC PREEMPT_RT çekirdeğinin gecikme (jitter) yaşamaması için harici Wi-Fi/Ethernet denetimi.");
                    });
                }
            });
        });
    }

    /// Controller messages, one line above the hint row.
    ///
    /// The controller is the only thing that reports why a program stopped, and
    /// those messages used to live only on the Machine page. The newest few are
    /// shown on every page, newest first, coloured by severity; the row
    /// disappears when the controller has said nothing.
    pub(super) fn messages_strip(&self, ui: &mut egui::Ui) {
        if self.snapshot.alarms.is_empty() {
            return;
        }
        let size = theme::font_size(9.5);
        let errors = self
            .snapshot
            .alarms
            .iter()
            .filter(|alarm| {
                matches!(
                    alarm.severity,
                    linc_core::model::AlarmSeverity::Error
                        | linc_core::model::AlarmSeverity::Critical
                )
            })
            .count();
        egui::containers::Panel::bottom("linc-messages")
            .exact_size(20.0)
            .frame(
                Frame::new()
                    .fill(theme::PANEL)
                    .inner_margin(Margin::symmetric(16, 2)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(if errors > 0 {
                            format!("MESSAGES · {errors} ERROR")
                        } else {
                            "MESSAGES".to_string()
                        })
                        .size(size)
                        .strong()
                        .color(if errors > 0 {
                            theme::ERROR
                        } else {
                            theme::MUTED
                        }),
                    );
                    ui.add_space(8.0);
                    for alarm in self.snapshot.alarms.iter().take(3) {
                        let colour = match alarm.severity {
                            linc_core::model::AlarmSeverity::Critical
                            | linc_core::model::AlarmSeverity::Error => theme::ERROR,
                            linc_core::model::AlarmSeverity::Warning => theme::WARNING,
                            linc_core::model::AlarmSeverity::Info => theme::MUTED,
                        };
                        // The strip is one row high: long messages are cut here
                        // and read in full in the messages dialog (M).
                        let text: String = alarm.message.chars().take(64).collect();
                        ui.label(RichText::new(text).size(size).color(colour));
                        ui.add_space(12.0);
                    }
                });
            });
    }

    /// The centre field of the bottom bar: the work position of every axis,
    /// then the same axes with the distance still to go.
    ///
    /// Two groups rather than interleaved pairs: X/Y/Z in one block reads as the
    /// position, and the DTG block lines up under it axis for axis. Painted on
    /// the Program page it replaces the single moving-axis DTG field the bar
    /// otherwise carries, so the distance is never shown twice.
    pub(super) fn coordinate_status_line(&self) -> String {
        let width = self.coordinate_field_width();
        let mut position = String::new();
        let mut dtg = String::new();
        for axis in self.axes() {
            // Same frame as the DRO cards: the bar and the cards disagreeing
            // about commanded vs actual would read as a position glitch.
            let Some((work, _machine, entry_dtg)) = screens::common::axis_display(self, &axis)
            else {
                continue;
            };
            position.push_str(&format!("{axis}{:+width$.3} ", work, width = width));
            dtg.push_str(&format!("{axis}{:+width$.3} ", entry_dtg, width = width));
        }
        format!("{}  DTG: {}", position.trim_end(), dtg.trim_end())
    }

    /// Characters in one coordinate field, sign and decimals included.
    ///
    /// Sized by the largest number that can appear - the machine envelope and
    /// the loaded program's extents - never by what is on screen: a field that
    /// widened when an axis crossed 10.000 made the whole line shuffle while the
    /// machine moved and the distance-to-go counted down.
    pub(super) fn coordinate_field_width(&self) -> usize {
        const DECIMALS: usize = 3;
        let mut extent: f64 = 0.0;
        for bounds in [
            self.machine_bounds(),
            self.program.last_valid_preview.bounds,
        ]
        .into_iter()
        .flatten()
        {
            for axis in 0..3 {
                extent = extent
                    .max(bounds.min[axis].abs())
                    .max(bounds.max[axis].abs());
            }
        }
        // Sign, the integer digits of the extent, the point, the decimals.
        1 + format!("{:.*}", DECIMALS, extent).len()
    }

    /// True where the bar shows the full coordinate line, so the single
    /// moving-axis DTG field steps aside instead of repeating it.
    pub(super) fn shows_coordinate_status(&self) -> bool {
        self.nav.page == Page::Program
    }

    /// Bottom bar: keyboard commands on the left, read-only machine status on
    /// the right. It carries no buttons.
    pub(super) fn bottom_bar(&mut self, ui: &mut egui::Ui) {
        let size = theme::font_size(11.0);
        ui.horizontal(|ui| {
            let field = |ui: &mut egui::Ui, text: String, color: egui::Color32| {
                ui.label(RichText::new(text).monospace().size(size).color(color));
            };
            // Left: what the machine is doing, in fields that appear only when
            // they have something to say. The bottom bar carries live machine
            // state alone; nothing here is an action.
            for (text, color) in self.status_fields() {
                field(ui, text, color);
                ui.add_space(16.0);
            }

            // The program's own numbers - the work position it is cutting to
            // and the distance left to get there - in the left-hand flow, after
            // the machine fields. Only on the Program page; the jog screens
            // carry the same figures in their own readout, and the bar is
            // read-only everywhere.
            if self.shows_coordinate_status() {
                let bar = ui.max_rect();
                let text = self.coordinate_status_line();
                let galley = ui
                    .painter()
                    .layout_no_wrap(text, FontId::monospace(size), theme::FG);
                // Never let it run into the left or the right block: the bar is
                // 1280 px wide at the minimum and those two need about 600.
                if galley.size().x + 24.0 < bar.width() * 0.5 {
                    // Left-aligned in the flow, right after the machine fields:
                    // the operator reads the bar left to right, and a centred
                    // block floats away from the line it belongs to.
                    let pos = egui::pos2(ui.cursor().min.x, bar.center().y - galley.size().y / 2.0);
                    ui.painter().galley(pos, galley, theme::FG);
                }
            }

            // Right: the read-only machine state.
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Never claim OK on stale data: the controller may be latched
                // while the last snapshot still says otherwise.
                let (estop_text, estop_color) = if self.snapshot.meta.stale {
                    ("E-STOP ?".to_string(), theme::WARNING)
                } else if self.snapshot.machine.estop {
                    ("E-STOP ACTIVE".to_string(), theme::ERROR)
                } else {
                    ("E-STOP OK".to_string(), theme::SUCCESS)
                };
                field(ui, estop_text, estop_color);
                ui.add_space(16.0);
                // Machine on/off sits immediately left of the E-stop state.
                // `halui.machine.is-on` is the authority; a stale snapshot
                // reports `?` rather than guessing. A running machine ON stays
                // steady; MACHINE OFF blinks so it cannot fade into a bar full
                // of green.
                let machine_is_off = !self.snapshot.machine.on && !self.snapshot.meta.stale;
                let (machine_text, machine_color) =
                    machine_status_field(self.snapshot.machine.on, self.snapshot.meta.stale);
                let machine_color = if machine_is_off {
                    blink_fault_color(ui)
                } else {
                    machine_color
                };
                field(ui, machine_text, machine_color);
                ui.add_space(16.0);
                // The wired probe input, right beside the machine state: the
                // operator proves the input by touching the probe with a
                // finger, which is a fault (`PROBE TRIPPED`) and not a
                // reading on the page that owns the probe. A profile without
                // a probe gets no dead field, and neither does a machine whose
                // spindle holds something else: the field claims a probe is
                // fitted, so it is drawn only while the probe tool is in the
                // spindle. Absent, not dimmed - the group is laid out from the
                // right, so the rest of the bar does not move.
                if probe_fitted(
                    self.profile.config.probe.enabled,
                    self.profile.config.probe.tool_number,
                    self.snapshot.tool_change.current_tool,
                ) {
                    let (probe_text, probe_color) = probe_input_field(
                        self.snapshot.probe.input_tripped,
                        self.snapshot.meta.stale,
                    );
                    field(ui, probe_text, probe_color);
                    ui.add_space(16.0);
                }
                // Protect is only worth a field when it is engaged: the off
                // state is the normal one and the row is space-limited.
                if self.program_protected() {
                    field(ui, "PROTECT ON".to_string(), theme::WARNING);
                    ui.add_space(16.0);
                }
                field(
                    ui,
                    override_readout(
                        self.snapshot.feed.feed_override,
                        self.snapshot.feed.rapid_override,
                        self.snapshot.feed.spindle_override,
                    ),
                    theme::MUTED,
                );
                if self.snapshot.limits_overridden {
                    ui.add_space(16.0);
                    limits_override_indicator(ui);
                }
                ui.add_space(16.0);
                field(
                    ui,
                    self.snapshot.panel.mode.label().to_string(),
                    if self.snapshot.panel.mode.is_valid() {
                        theme::ACCENT
                    } else {
                        theme::ERROR
                    },
                );
            });
        });
    }

    pub(super) fn banners(&mut self, _ui: &mut egui::Ui) {
        // Status alerts are rendered as compact, high-visibility badges in the top bar (`top_bar`),
        // preventing vertical layout shifts and maximizing screen area for 3D preview and toolpaths.
    }

    pub(super) fn status_badges(&mut self, ui: &mut egui::Ui) {
        let size = theme::font_size(10.0);
        if self.snapshot.limits_overridden {
            let label = linc_core::i18n::t("shell.badges.limits_override");
            let tooltip = linc_core::i18n::t("shell.badges.limits_override_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(84.0, 26.0), Sense::hover());
            ui.painter().rect_filled(rect, 8, Color32::from_rgba_unmultiplied(245, 158, 11, 40));
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, theme::COCKPIT_AMBER), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(theme::font_size(10.5)), theme::COCKPIT_AMBER);
            resp.on_hover_text(tooltip);
        }
        if self.snapshot.machine.estop {
            let label = linc_core::i18n::t("shell.badges.estop");
            let tooltip = linc_core::i18n::t("shell.badges.estop_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(78.0, 26.0), Sense::hover());
            ui.painter().rect_filled(rect, 8, theme::COCKPIT_RED);
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, Color32::from_rgb(0xf8, 0x71, 0x71)), StrokeKind::Inside);
            // Draw mini mushroom stop icon
            let c = rect.center();
            let icon_c = egui::pos2(rect.left() + 15.0, c.y);
            ui.painter().circle_filled(egui::pos2(icon_c.x, icon_c.y - 3.0), 4.0, Color32::WHITE);
            ui.painter().rect_filled(egui::Rect::from_center_size(egui::pos2(icon_c.x, icon_c.y + 2.0), Vec2::new(4.0, 4.0)), 1, Color32::WHITE);
            ui.painter().rect_filled(egui::Rect::from_center_size(egui::pos2(icon_c.x, icon_c.y + 5.0), Vec2::new(8.0, 2.0)), 1, Color32::WHITE);
            ui.painter().text(egui::pos2(rect.left() + 46.0, c.y), Align2::CENTER_CENTER, label, FontId::monospace(theme::font_size(11.0)), Color32::WHITE);
            resp.on_hover_text(tooltip);
        }
        if let Some(fault) = self.snapshot.probe_trip.clone() {
            ui.add_space(6.0);
            let label = linc_core::i18n::t("shell.badges.probe_trip");
            let trip_desc = probe_trip_banner_text(&fault);
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(88.0, 26.0), Sense::click());
            let bg = if resp.hovered() { Color32::from_rgb(255, 80, 80) } else { theme::COCKPIT_RED };
            ui.painter().rect_filled(rect, 8, bg);
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, Color32::from_rgb(255, 150, 150)), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(size), Color32::WHITE);
            if resp.clicked() {
                self.clear_probe_trip();
            }
            resp.on_hover_ui(|ui| {
                ui.label(RichText::new(linc_core::i18n::t("shell.badges.probe_trip_tooltip")).strong().color(theme::COCKPIT_RED));
                ui.label(RichText::new(trip_desc).size(11.0));
                ui.label(RichText::new("Sıfırlamak için tıklayın").size(10.0).color(theme::COCKPIT_TEXT_MUTED));
            });
        }
        if self.panel_offline() {
            ui.add_space(6.0);
            let label = linc_core::i18n::t("shell.badges.panel_offline");
            let tooltip = linc_core::i18n::t("shell.badges.panel_offline_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(90.0, 26.0), Sense::hover());
            ui.painter().rect_filled(rect, 8, theme::COCKPIT_RED);
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, Color32::from_rgb(255, 120, 120)), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(size), Color32::WHITE);
            resp.on_hover_text(tooltip);
        } else if self.invalid_mode() {
            ui.add_space(6.0);
            let label = linc_core::i18n::t("shell.badges.panel_invalid");
            let tooltip = linc_core::i18n::t("shell.badges.panel_invalid_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(80.0, 26.0), Sense::hover());
            ui.painter().rect_filled(rect, 8, theme::COCKPIT_RED);
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, Color32::from_rgb(255, 120, 120)), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(size), Color32::WHITE);
            resp.on_hover_text(tooltip);
        }
        if self.program_protected()
            && matches!(
                self.nav.page,
                Page::Program | Page::Offsets | Page::Tools | Page::Probe | Page::Files
            )
        {
            ui.add_space(6.0);
            let label = linc_core::i18n::t("shell.badges.protect");
            let tooltip = linc_core::i18n::t("shell.badges.protect_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(56.0, 26.0), Sense::hover());
            ui.painter().rect_filled(rect, 8, Color32::from_rgba_unmultiplied(245, 158, 11, 40));
            ui.painter().rect_stroke(rect, 8, Stroke::new(1.0, theme::COCKPIT_AMBER), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(size), theme::COCKPIT_AMBER);
            resp.on_hover_text(tooltip);
        }
        if self.nav.page == Page::Program && self.program.external_change {
            ui.add_space(8.0);
            let label = linc_core::i18n::t("shell.badges.ext_change");
            let tooltip = linc_core::i18n::t("shell.badges.ext_change_tooltip");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(82.0, 18.0), Sense::hover());
            ui.painter().rect_filled(rect, 3, theme::WARNING);
            ui.painter().rect_stroke(rect, 3, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::monospace(size), Color32::from_rgb(0x1a, 0x12, 0x04));
            resp.on_hover_text(tooltip);
        }
    }
}

/// State the bars paint: connection and protect edges, the lathe readout mode,
/// and the counters the alarm strip compares frame to frame.
pub(crate) struct ChromeUi {
    pub quit_requested: bool,
    pub last_snapshot_ms: u64,
    /// Highest alarm id the operator has been told about. Controller errors used
    /// to appear only in the Machine page's message card, so a program that
    /// aborted looked like it had simply jumped to the end.
    pub alarm_id_seen: u64,
    pub last_connection: Option<bool>,
    pub last_protect: Option<bool>,
}
