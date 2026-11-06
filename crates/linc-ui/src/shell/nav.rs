//! Page routing and the navigation column: which page the panel mode owns,
//! where a page returns to, and the geometry of a nav row.

use super::*;

pub(super) fn draw_nav_icon(painter: &egui::Painter, rect: egui::Rect, page: Page, color: Color32) {
    let s = egui::Stroke::new(1.4, color);
    let c = rect.center();
    match page {
        Page::Machine => {
            // Hız Kadranı / Takometre Yayı + İbre (HTML Prototip ⏱)
            let r = 7.0;
            let start_angle = std::f32::consts::PI * 0.75; // 135 derece
            let end_angle = std::f32::consts::PI * 2.25;   // 405 derece
            let steps = 14;
            let mut pts = Vec::with_capacity(steps + 1);
            for i in 0..=steps {
                let a = start_angle + (end_angle - start_angle) * (i as f32 / steps as f32);
                let (sin, cos) = a.sin_cos();
                pts.push(egui::pos2(c.x + cos * r, c.y + sin * r));
            }
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], s);
            }
            // Uç çentikleri
            let (s_sin, s_cos) = start_angle.sin_cos();
            painter.line_segment(
                [egui::pos2(c.x + s_cos * (r - 2.0), c.y + s_sin * (r - 2.0)), egui::pos2(c.x + s_cos * r, c.y + s_sin * r)],
                s,
            );
            let (e_sin, e_cos) = end_angle.sin_cos();
            painter.line_segment(
                [egui::pos2(c.x + e_cos * (r - 2.0), c.y + e_sin * (r - 2.0)), egui::pos2(c.x + e_cos * r, c.y + e_sin * r)],
                s,
            );
            // Sağ üste doğru ibre (~45 derece)
            let needle_angle = -std::f32::consts::FRAC_PI_4;
            let (n_sin, n_cos) = needle_angle.sin_cos();
            painter.line_segment(
                [c, egui::pos2(c.x + n_cos * (r - 1.5), c.y + n_sin * (r - 1.5))],
                egui::Stroke::new(1.8, color),
            );
            painter.circle_filled(c, 1.8, color);
        }
        Page::Program => {
            // G-Kodu Doküman Sayfası (Kıvrımlı Köşe) + Sağ Alt Dişli Çark (Referans Görsel Birebir 📄⚙)
            let left = rect.left() + 2.5;
            let right = rect.right() - 2.5;
            let top = rect.top() + 1.0;
            let bottom = rect.bottom() - 1.0;
            let fold = 3.5;

            // Doküman dış çizgileri
            painter.line_segment([egui::pos2(left, top), egui::pos2(right - fold, top)], s);
            painter.line_segment([egui::pos2(right - fold, top), egui::pos2(right, top + fold)], s);
            painter.line_segment([egui::pos2(right, top + fold), egui::pos2(right, bottom - 5.5)], s);
            painter.line_segment([egui::pos2(left, top), egui::pos2(left, bottom)], s);
            painter.line_segment([egui::pos2(left, bottom), egui::pos2(right - 5.5, bottom)], s);

            // Kıvrım üçgeni
            painter.line_segment([egui::pos2(right - fold, top), egui::pos2(right - fold, top + fold)], s);
            painter.line_segment([egui::pos2(right - fold, top + fold), egui::pos2(right, top + fold)], s);

            // G-Kodu Satırları
            let ls = egui::Stroke::new(1.1, color);
            painter.line_segment([egui::pos2(left + 2.5, top + 5.0), egui::pos2(left + 5.5, top + 5.0)], ls);
            painter.line_segment([egui::pos2(left + 2.5, top + 7.8), egui::pos2(left + 7.5, top + 7.8)], ls);
            painter.line_segment([egui::pos2(left + 2.5, top + 10.6), egui::pos2(left + 5.0, top + 10.6)], ls);

            // Sağ alt 6-dişli çark
            let gc = egui::pos2(right - 2.0, bottom - 2.0);
            let gr = 3.2;
            painter.circle_stroke(gc, gr, egui::Stroke::new(1.2, color));
            painter.circle_filled(gc, 1.2, color);
            for i in 0..6 {
                let a = i as f32 * std::f32::consts::PI / 3.0;
                let (sin, cos) = a.sin_cos();
                painter.line_segment(
                    [egui::pos2(gc.x + cos * (gr - 0.4), gc.y + sin * (gr - 0.4)),
                     egui::pos2(gc.x + cos * (gr + 1.5), gc.y + sin * (gr + 1.5))],
                    egui::Stroke::new(1.3, color),
                );
            }
        }
        Page::Tools => {
            // Endüstriyel Açık Ağız İngiliz Anahtarı (HTML Prototip 🔧)
            let handle_start = egui::pos2(c.x - 5.5, c.y + 5.5);
            let head_center = egui::pos2(c.x + 2.0, c.y - 2.0);
            // Anahtar sapı
            painter.line_segment([handle_start, head_center], egui::Stroke::new(2.3, color));
            // Anahtar açık ağız kavis ve çeneleri
            let _jaw_angle = -std::f32::consts::FRAC_PI_4; // 45 derece sap açısı
            let p_left = egui::pos2(head_center.x - 2.8, head_center.y - 2.8);
            let p_right = egui::pos2(head_center.x + 2.8, head_center.y + 2.8);
            let jaw_top_left = egui::pos2(head_center.x - 1.2, head_center.y - 4.8);
            let jaw_top_right = egui::pos2(head_center.x + 4.8, head_center.y + 1.2);
            let jaw_inner = egui::pos2(head_center.x + 0.5, head_center.y - 0.5);

            painter.line_segment([p_left, jaw_top_left], egui::Stroke::new(1.5, color));
            painter.line_segment([jaw_top_left, jaw_inner], egui::Stroke::new(1.4, color));
            painter.line_segment([jaw_inner, jaw_top_right], egui::Stroke::new(1.4, color));
            painter.line_segment([jaw_top_right, p_right], egui::Stroke::new(1.5, color));
            painter.line_segment([p_left, p_right], egui::Stroke::new(1.5, color));
        }
        Page::Offsets => {
            // WCS Datum Parça Sıfırı Hedefi (⊕) (HTML Prototip ⊕)
            painter.circle_stroke(c, 6.2, s);
            painter.circle_filled(c, 1.8, color);
            painter.line_segment([egui::pos2(c.x, rect.top() + 1.5), egui::pos2(c.x, rect.bottom() - 1.5)], s);
            painter.line_segment([egui::pos2(rect.left() + 1.5, c.y), egui::pos2(rect.right() - 1.5, c.y)], s);
        }
        Page::Diagnostics => {
            // Endüstriyel Osiloskop & HAL Sinyal Dalga Formu (HTML Prototip ~●~)
            let pts = [
                egui::pos2(rect.left() + 1.5, c.y),
                egui::pos2(c.x - 4.5, c.y),
                egui::pos2(c.x - 2.0, c.y - 5.5),
                egui::pos2(c.x + 1.0, c.y + 4.5),
                egui::pos2(c.x + 3.5, c.y),
                egui::pos2(rect.right() - 1.5, c.y),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], egui::Stroke::new(1.4, color));
            }
            painter.circle_filled(egui::pos2(c.x - 2.0, c.y - 5.5), 1.6, color);
        }
        Page::Help => {
            // Daire İçi Soru İşareti: (?) (HTML Prototip (?))
            painter.circle_stroke(c, 6.5, s);
            painter.text(
                egui::pos2(c.x, c.y - 0.5),
                Align2::CENTER_CENTER,
                "?",
                FontId::proportional(theme::font_size(10.0)),
                color,
            );
        }
        Page::Files | Page::Probe | Page::Homing | Page::Manual => {
            painter.rect_stroke(rect.shrink(3.0), 2.0, s, StrokeKind::Inside);
        }
    }
}

/// Parse a page name as the diagnostic tools spell it. Accepts the navigation
/// label and the enum-ish spelling, so `G-CODE HELP` and `HELP` both work.
pub fn page_from_name(name: &str) -> Option<Page> {
    let wanted = name
        .trim()
        .to_ascii_uppercase()
        .replace(['-', '_', ' '], " ");
    for page in [
        Page::Machine,
        Page::Program,
        Page::Files,
        Page::Tools,
        Page::Offsets,
        Page::Diagnostics,
        Page::Help,
        Page::Probe,
        Page::Homing,
        Page::Manual,
    ] {
        let label = page.label().to_ascii_uppercase().replace(['-', '_'], " ");
        if label == wanted || format!("{page:?}").to_ascii_uppercase() == wanted {
            return Some(page);
        }
    }
    None
}

/// Navigation entries in display order, with their label and keycap hint.
///
/// Which of these actually appear is `[ui] pages` in the profile, so a machine
/// can hide the pages it does not use (a machine with no changer can hide
/// TOOLS, for example) without removing the screen from the binary.
/// Ordered to match the keyboard: MAIN (F5), FILE (F6), TOOL (F7), OFFSET
/// (F8). A page that has no function key follows the ones that do.
///
/// DIAGNOSTICS and G-CODE HELP are listed too: `configs/base.toml` names all
/// seven, so the operator can reach the diagnostics and help pages with the
/// arrows and Enter as well as with their `[keys]` chords (Ctrl+D, Ctrl+H).
/// They draw as one-line rows without an icon (see [`NAV_PRIMARY`]).
///
/// The keycap is the panel's printed label; [`nav_keycap`] swaps it for the
/// binding a normal keyboard sends when the profile asks for PC legends.
pub const NAV_ITEMS: [(Page, &str, &str); 6] = [
    (Page::Machine, "MACHINE", "MAIN"),
    (Page::Program, "PROGRAM", "FILE"),
    (Page::Tools, "TOOLS", "TOOL"),
    (Page::Offsets, "OFFSETS", "PROBE"),
    (Page::Diagnostics, "DIAG", "Ctrl+D"),
    (Page::Help, "G-CODE", "Ctrl+H"),
];

/// The key hint a navigation row prints.
///
/// `[ui] keyboard = "panel"` keeps the CNC panel's printed keycaps; `"pc"`
/// pairs the panel name with the binding a normal keyboard sends
/// (`MAIN · F5`), so an operator on an ordinary keyboard can read both. The
/// OFFSETS row prints `PROBE` on the panel, but `PROBE · F8` is too wide for
/// the pill, so its PC legend abbreviates to `OFFS · F8`. DIAG and G-CODE
/// already print their chords and are left alone either way.
#[allow(dead_code)]
fn nav_keycap(app: &LincApp, page: Page, panel: &str) -> String {
    if !app.pc_keyboard() {
        return panel.to_string();
    }
    let (name, binding) = match page {
        Page::Machine => (panel, app.prefs.keymap.machine.label()),
        Page::Program => (panel, app.prefs.keymap.program.label()),
        Page::Tools => (panel, app.prefs.keymap.tools.label()),
        Page::Offsets => ("OFFS", app.prefs.keymap.offsets.label()),
        _ => return panel.to_string(),
    };
    format!("{name} · {binding}")
}

/// The navigation rows that carry an icon and a two-line label.
///
/// The four entries with a function key do: they are what the operator's hand
/// finds on the panel. DIAGNOSTICS and G-CODE HELP are reached by a chord or by
/// the arrows, so they draw as one-line rows without an icon — a third of the
/// attention, and in a six-row list the vertical space.
pub(super) const NAV_PRIMARY: [Page; 4] =
    [Page::Machine, Page::Program, Page::Tools, Page::Offsets];

/// Geometry of one navigation row: the target the operator gets and the pill
/// painted inside it.
pub struct NavRowGeometry {
    /// Interaction height. Never below [`theme::MIN_TARGET`].
    pub height: f32,
    /// Painted pill. Shorter than `height`: what shrank is the padding, not
    /// the target.
    pub pill: f32,
    /// Gap above this row. A one-line row uses half the primary gap so two
    /// consecutive one-line rows do not pay the padding twice.
    pub gap: f32,
    /// An icon beside a two-line label.
    pub icon: bool,
}

pub fn nav_row_geometry(page: Page) -> NavRowGeometry {
    if NAV_PRIMARY.contains(&page) {
        NavRowGeometry {
            height: theme::NAV_ROW_HEIGHT,
            pill: theme::NAV_PILL_PRIMARY,
            gap: theme::NAV_ROW_GAP,
            icon: true,
        }
    } else {
        NavRowGeometry {
            height: theme::NAV_ROW_HEIGHT_SECONDARY,
            pill: theme::NAV_PILL_SECONDARY,
            gap: theme::NAV_ROW_GAP_SECONDARY,
            icon: false,
        }
    }
}

/// Navigation entries to show, from `[ui] pages`.
///
/// An empty list means "all of them", so a profile that never mentions the key
/// still gets the full navigation.
pub(super) fn nav_pages(configured: &[String]) -> Vec<Page> {
    if configured.is_empty() {
        return NAV_ITEMS.iter().map(|(page, _, _)| *page).collect();
    }
    configured.iter().map(|key| page_from_config(key)).collect()
}

/// Stable preferences key for a page. Mode-driven pages are never persisted.
pub(super) fn page_key(page: Page) -> &'static str {
    match page {
        Page::Machine => "machine",
        Page::Program => "program",
        Page::Offsets => "offsets",
        Page::Tools => "tools",
        Page::Diagnostics => "diagnostics",
        Page::Help => "help",
        Page::Files => "files",
        Page::Probe => "probe",
        Page::Homing => "homing",
        Page::Manual => "manual",
    }
}

pub(super) fn page_from_config(value: &str) -> Page {
    match value.to_ascii_lowercase().as_str() {
        "program" => Page::Program,
        "offsets" => Page::Offsets,
        "tools" => Page::Tools,
        "diagnostics" => Page::Diagnostics,
        "help" => Page::Help,
        "files" => Page::Files,
        "probe" => Page::Probe,
        _ => Page::Machine,
    }
}

pub fn mode_page(mode: &PanelModeValidity) -> Option<Page> {
    match mode.mode()? {
        // MDI has no page of its own: it opens MAIN, where the mode swaps
        // the program card for the MDI pane.
        PanelMode::Mdi => Some(Page::Machine),
        PanelMode::Home | PanelMode::ProgHome => Some(Page::Homing),
        PanelMode::Jog | PanelMode::Inc | PanelMode::Mpg => Some(Page::Manual),
        PanelMode::Edit => Some(Page::Program),
        PanelMode::Auto => None,
    }
}

impl LincApp {
    /// The screen the panel's current mode owns.
    ///
    /// `MAIN` is the operator asking for "the screen for this mode", not for the
    /// AUTO dashboard: in EDIT that screen is the program editor, in
    /// JOG/INC/MPG it is the manual page and in HOME/PROG.HOME it is the homing
    /// page. The shared `mode_page` mapping answers `None` for AUTO (whose
    /// screen is MAIN) and for an unknown or zero mode, which fails closed onto
    /// the dashboard as well.
    pub fn mode_screen(&self) -> Page {
        mode_page(&self.snapshot.panel.mode).unwrap_or(Page::Machine)
    }

    /// `MAIN` (F5): go to the mode's page, without the Program row's
    /// second meaning of "open the file browser" when Program is already in
    /// front.
    pub fn request_mode_page(&mut self) {
        let page = self.mode_screen();
        if page == self.nav.page {
            return;
        }
        self.request_page(page);
    }

    pub fn set_page(&mut self, page: Page) {
        self.remember_page(page);
        if page == Page::Help && self.nav.page != Page::Help {
            // Captured before the assignment below, which would otherwise make
            // the Help page its own return destination.
            self.nav.help_return_page = if self.nav.page.is_mode_driven() {
                self.nav.last_non_mode_page
            } else {
                self.nav.page
            };
        }
        if page == Page::Diagnostics && self.nav.page != Page::Diagnostics {
            self.nav.diagnostics_return_page = if self.nav.page.is_mode_driven() {
                self.nav.last_non_mode_page
            } else {
                self.nav.page
            };
        }
        if page == Page::Files {
            self.nav.files_return_page = if self.nav.page == Page::Files {
                Page::Program
            } else {
                self.nav.page
            };
            // The program list is what the operator came for. The USB pane
            // keeps the focus only for the visit that put it there.
            self.files.usb_focused = false;
            self.files.refresh();
        }
        if page == Page::Probe {
            self.nav.homing_return_page = Page::Offsets;
            // Every entry path starts with nothing selected: Esc from Files
            // returns to the tab, not to a routine Enter would run. The tab
            // itself is preserved; only F8 selects the first one.
            screens::probe::clear_selection(self);
        }
        if !page.is_mode_driven() {
            self.nav.last_non_mode_page = page;
        }
        self.nav.page = page;
        self.nav.focus_region = FocusRegion::Page;
        self.nav.page_focus_requested = true;
        if page == Page::Tools {
            // The table takes the keyboard as soon as the page opens: nothing
            // else on the page holds focus, and the cursor starts at the first
            // editable cell of the selected row.
            self.tools.editing = false;
            self.tools.edit_buffer.clear();
            if self.tools.selected_column == 0 {
                self.tools.selected_column = 1;
            }
        } else {
            // The tool dialogs belong to the page. Left open across a page
            // change they would keep `overlay_open` true with nothing on
            // screen, and every key would stay with a dialog that is not drawn.
            self.tools.number = None;
            self.tools.position = None;
        }
        if page == Page::Help {
            self.help.ensure_loaded();
        }
        if page == Page::Program && self.program.view == ProgramView::Code && self.editing_allowed()
        {
            self.program.focus_editor = true;
        }
        if page == Page::Machine {
            // Entering MAIN while the panel sits in MDI is also the pane
            // appearing, so it takes the keyboard like a mode entry does. It
            // only asks here: a click elsewhere on the page gives it up again
            // and the flag is one-shot.
            self.mdi.focus = self.snapshot.panel.mode.mode() == Some(PanelMode::Mdi);
        }
    }

    pub fn request_page(&mut self, page: Page) {
        // The PROGRAM row is the FILE key: on the page already in front it
        // opens the file browser, and the row on the Files page brings the
        // program back. CODE/SPLIT is the square key's command, not this row's.
        if page == Page::Program && self.nav.page == Page::Program {
            self.request_files();
            return;
        }
        if self.program.editor.dirty && self.nav.page == Page::Program && page != Page::Program {
            self.overlay.modal = Some(Modal::ConfirmDiscard(PendingAction::Navigate(page)));
        } else {
            self.set_page(page);
        }
    }

    /// The OFFSETS row and `F8` share one route: the offset table first, then
    /// the probe tabs, then back to the table. On the last tab the next press
    /// returns to the table, so repeating the command walks every tab and
    /// arrives where it started.
    pub fn request_offsets(&mut self) {
        if self.nav.page == Page::Probe {
            if linc_core::probe::ProbeCategory::ALL.last().copied() == Some(self.probe.category) {
                self.set_page(Page::Offsets);
            } else {
                screens::probe::category_step(self, true);
            }
            return;
        }
        if self.nav.page == Page::Offsets
            && self.snapshot.probe.enabled
            && self.profile.variant == MachineVariant::Mill
        {
            screens::probe::enter_first(self);
            self.set_page(Page::Probe);
            return;
        }
        self.request_page(Page::Offsets);
    }

    /// The DIAG row and `Ctrl+D` share one route: the page first, then the
    /// visible tabs in strip order, wrapping back to the first. The strip the
    /// operator sees is the cycle, so a profile without the turret tab never
    /// lands on it.
    pub fn request_diagnostics(&mut self) {
        if self.nav.page == Page::Diagnostics {
            let tabs = DiagnosticsTab::visible(&self.profile);
            let index = tabs
                .iter()
                .position(|tab| *tab == self.diagnostics.tab)
                .unwrap_or(0);
            self.diagnostics.tab = tabs[(index + 1) % tabs.len()];
            return;
        }
        self.request_page(Page::Diagnostics);
    }

    /// A row activated from the sidebar (click or Enter) has the row's own
    /// meaning, which for OFFSETS is the probe route above and for DIAG is the
    /// tab cycle.
    pub(super) fn request_nav_page(&mut self, page: Page) {
        if page == Page::Offsets {
            self.request_offsets();
        } else if page == Page::Diagnostics {
            self.request_diagnostics();
        } else {
            self.request_page(page);
        }
    }

    pub fn request_files(&mut self) {
        if self.program.editor.dirty && self.nav.page == Page::Program {
            self.overlay.modal = Some(Modal::ConfirmDiscard(PendingAction::OpenFiles));
        } else {
            self.open_files();
        }
    }

    pub fn open_files(&mut self) {
        self.nav.files_return_page = if self.nav.page == Page::Files {
            Page::Program
        } else {
            self.nav.page
        };
        self.files.refresh();
        self.set_page(Page::Files);
    }

    /// Remember the last operator-selected, non-mode-driven page.
    pub(super) fn remember_page(&mut self, page: Page) {
        if page.is_mode_driven() {
            return;
        }
        let key = page_key(page);
        if self.prefs.preferences.last_page.as_deref() != Some(key) {
            self.prefs.preferences.last_page = Some(key.to_string());
            self.prefs.dirty = true;
        }
    }

    /// Enter a page that is coming back after its mode went away. The
    /// destination policy is unchanged (`last_non_mode_page` still decides),
    /// but a restored Probe page is an entry like any other: nothing may come
    /// back selected for Enter to run.
    pub(super) fn restore_page(&mut self, page: Page) {
        if page == Page::Probe {
            screens::probe::clear_selection(self);
        }
        self.nav.page = page;
    }

    pub(super) fn on_mode_change(&mut self, mode: PanelModeValidity) {
        let Some(new_mode) = mode.mode() else {
            if self.nav.page.is_mode_driven() {
                self.restore_page(self.nav.last_non_mode_page);
            }
            return;
        };
        let prior_mode = self
            .nav
            .last_mode
            .as_ref()
            .and_then(PanelModeValidity::mode);
        // Leaving EDIT is the one moment unsaved work can become unreachable:
        // the controller refuses a save outside EDIT, so AUTO can only ever run
        // the loaded file. Say so here, before the operator presses CYCLE START
        // on a program that will not run.
        let left_edit = prior_mode == Some(PanelMode::Edit) && new_mode != PanelMode::Edit;
        if left_edit && self.program.editor.dirty && self.overlay.modal.is_none() {
            self.overlay.modal = Some(Modal::UnsavedOnLeave);
        }
        if prior_mode.map(PanelMode::is_manual).unwrap_or(false)
            && !new_mode.is_manual()
            && self.nav.page == Page::Manual
        {
            self.restore_page(self.nav.last_non_mode_page);
        }
        if matches!(
            prior_mode,
            Some(PanelMode::Home) | Some(PanelMode::ProgHome)
        ) && !matches!(new_mode, PanelMode::Home | PanelMode::ProgHome)
            && self.nav.page == Page::Homing
        {
            self.restore_page(self.nav.last_non_mode_page);
        }
        match new_mode {
            PanelMode::Mdi => {
                // MDI is not a page: the panel position opens MAIN, whose
                // program card the mode swaps for the MDI pane.
                self.nav.page = Page::Machine;
                if self.profile.config.mdi.auto_focus {
                    self.mdi.focus = true;
                }
            }
            PanelMode::Home | PanelMode::ProgHome => {
                if self.nav.page != Page::Homing {
                    self.nav.homing_return_page = if self.nav.page.is_mode_driven() {
                        self.nav.last_non_mode_page
                    } else {
                        self.nav.page
                    };
                }
                self.nav.page = Page::Homing;
            }
            PanelMode::Jog | PanelMode::Inc | PanelMode::Mpg => {
                if self.nav.page != Page::Manual {
                    self.nav.last_non_mode_page = if self.nav.page.is_mode_driven() {
                        self.nav.last_non_mode_page
                    } else {
                        self.nav.page
                    };
                }
                self.manual.view = if self.profile.variant == MachineVariant::Lathe {
                    linc_view::ViewMode::LatheXz
                } else {
                    self.manual.view
                };
                self.nav.page = Page::Manual;
            }
            PanelMode::Edit => {
                if self.nav.page != Page::Program {
                    self.nav.homing_return_page = self.nav.page;
                }
                self.nav.page = Page::Program;
                self.program.view = ProgramView::Code;
                self.program.focus_editor = self.program_editing_allowed();
            }
            PanelMode::Auto => {
                // Selecting AUTO brings up MAIN from any page: the run is
                // started from the machine page, whose toolpath and program
                // cards carry the active line. Leaving MDI swaps the pane back
                // for the normal read-only program card. This runs on the mode
                // change only, so navigating away afterwards is left alone.
                self.nav.page = Page::Machine;
            }
        }
    }

    pub fn focus_nav(&mut self) {
        self.nav.focus_region = FocusRegion::Nav;
        self.nav.nav_focus = self
            .nav
            .pages
            .iter()
            .position(|page| *page == self.nav.page)
            .unwrap_or(0);
    }

    pub(super) fn draw_nav(&mut self, ui: &mut egui::Ui) {
        let shortcuts = self.key_hints();
        let cur_loc = linc_core::i18n::current_locale();
        let tr_active = cur_loc == linc_core::i18n::Locale::Tr;
        let en_active = cur_loc == linc_core::i18n::Locale::En;
        let is_sim = self.is_sim();
        let is_open = self.sim_panel_should_show();
        let machine_name = self.profile.config.machine.name.clone();
        let fps_val = self.fps;

        let nav_pages = self.nav.pages.clone();
        let current_page = self.nav.page;
        let nav_focus = self.nav.nav_focus;
        let focus_nav = self.nav.focus_region == FocusRegion::Nav;
        let pc_keyboard = self.pc_keyboard();
        let km_machine = self.prefs.keymap.machine.label().to_string();
        let km_prog = self.prefs.keymap.program.label().to_string();
        let km_tools = self.prefs.keymap.tools.label().to_string();
        let km_offsets = self.prefs.keymap.offsets.label().to_string();

        let mut nav_action: Option<(usize, Page)> = None;
        let mut sim_toggle = false;
        let mut open_help_doc = false;

        // egui 0.37 Sides layout: guarantees the right system cluster is sized first,
        // and left navigation tabs take the remaining bounded area with zero overlapping.
        egui::containers::Sides::new()
            .height(30.0)
            .shrink_left()
            .show(
                ui,
                |ui| {
                    // LEFT: Single-line Industrial Navigation Tabs
                    egui::ScrollArea::horizontal()
                        .id_salt("navigation-tabs")
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                for (index, page) in nav_pages.iter().enumerate() {
                                    let Some((_, label, keycap_hint)) =
                                        NAV_ITEMS.iter().find(|(entry, _, _)| entry == page)
                                    else {
                                        continue;
                                    };
                                    let keycap = if !pc_keyboard {
                                        keycap_hint.to_string()
                                    } else {
                                        match *page {
                                            Page::Machine => format!("{keycap_hint} · {km_machine}"),
                                            Page::Program => format!("{keycap_hint} · {km_prog}"),
                                            Page::Tools => format!("{keycap_hint} · {km_tools}"),
                                            Page::Offsets => format!("OFFS · {km_offsets}"),
                                            _ => keycap_hint.to_string(),
                                        }
                                    };
                                    let selected = *page == current_page
                                        || (*page == Page::Offsets && current_page == Page::Probe);
                                    let focused = focus_nav && nav_focus == index;

                                    let card_bg = if selected {
                                        theme::COCKPIT_BLUE
                                    } else if focused {
                                        theme::COCKPIT_CARD_HOVER
                                    } else {
                                        theme::COCKPIT_CARD
                                    };
                                    let card_border = if selected {
                                        Color32::from_rgb(0x60, 0xa5, 0xfa)
                                    } else if focused {
                                        theme::ACCENT
                                    } else {
                                        theme::COCKPIT_BORDER
                                    };

                                    let icon_color = if selected {
                                        Color32::WHITE
                                    } else {
                                        theme::COCKPIT_TEXT_MUTED
                                    };
                                    let label_color = if selected {
                                        Color32::WHITE
                                    } else {
                                        theme::COCKPIT_TEXT_MAIN
                                    };

                                    let frame = egui::Frame::new()
                                        .fill(card_bg)
                                        .stroke(Stroke::new(1.0, card_border))
                                        .corner_radius(8)
                                        .inner_margin(egui::Margin::symmetric(8, 4));

                                    let frame_resp = frame.show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 6.0;

                                            // 1. Vector Icon
                                            let (icon_rect, _) = ui.allocate_exact_size(
                                                Vec2::splat(16.0),
                                                Sense::hover(),
                                            );
                                            draw_nav_icon(ui.painter(), icon_rect, *page, icon_color);

                                            // 2. Tab Label (native egui layout prevents any text collision)
                                            ui.label(
                                                RichText::new(*label)
                                                    .size(theme::font_size(10.5))
                                                    .strong()
                                                    .color(label_color),
                                            );

                                            // 3. Keycap Badge
                                            if selected {
                                                egui::Frame::new()
                                                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 45))
                                                    .corner_radius(4)
                                                    .inner_margin(egui::Margin::symmetric(4, 1))
                                                    .show(ui, |ui| {
                                                        ui.label(
                                                            RichText::new(&keycap)
                                                                .monospace()
                                                                .size(theme::font_size(8.5))
                                                                .color(Color32::WHITE),
                                                        );
                                                    });
                                            } else {
                                                egui::Frame::new()
                                                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 12))
                                                    .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 25)))
                                                    .corner_radius(4)
                                                    .inner_margin(egui::Margin::symmetric(4, 1))
                                                    .show(ui, |ui| {
                                                        ui.label(
                                                            RichText::new(&keycap)
                                                                .monospace()
                                                                .size(theme::font_size(8.0))
                                                                .color(theme::COCKPIT_TEXT_DIM),
                                                        );
                                                    });
                                            }
                                        });
                                    });

                                    let btn_resp = ui.interact(
                                        frame_resp.response.rect,
                                        ui.scope_id().with(index),
                                        Sense::click(),
                                    );
                                    if btn_resp.hovered() && !selected {
                                        ui.painter().rect_stroke(
                                            frame_resp.response.rect,
                                            8,
                                            Stroke::new(1.0, theme::COCKPIT_BORDER_ACTIVE),
                                            StrokeKind::Inside,
                                        );
                                    }
                                    if btn_resp.clicked() {
                                        nav_action = Some((index, *page));
                                    }
                                }
                            });
                        });
                },
                |ui| {
                    // RIGHT: System Cluster (Right to Left in SidesKind::ShrinkLeft)
                    ui.spacing_mut().item_spacing.x = 8.0;

                    // 1. Canlı Saat (Digital Cockpit Clock)
                    ui.label(
                        RichText::new(format_clock(now_ms()))
                            .monospace()
                            .size(theme::font_size(11.5))
                            .strong()
                            .color(theme::COCKPIT_TEXT_MAIN),
                    );

                    // Ayırıcı çizgi
                    let (sep_rect, _) = ui.allocate_exact_size(Vec2::new(1.0, 18.0), Sense::hover());
                    ui.painter().line_segment(
                        [egui::pos2(sep_rect.center().x, sep_rect.top()), egui::pos2(sep_rect.center().x, sep_rect.bottom())],
                        Stroke::new(1.0, theme::COCKPIT_BORDER),
                    );

                    // 2. Dil Seçici Kapsülü ([TR] / [EN])
                    let (lang_rect, _) = ui.allocate_exact_size(Vec2::new(56.0, 26.0), Sense::hover());
                    ui.painter().rect_filled(lang_rect, 6, theme::COCKPIT_CARD);
                    ui.painter().rect_stroke(lang_rect, 6, Stroke::new(1.0, theme::COCKPIT_BORDER), StrokeKind::Inside);

                    let tr_rect = egui::Rect::from_min_max(lang_rect.min + egui::vec2(1.5, 1.5), egui::pos2(lang_rect.center().x - 0.5, lang_rect.max.y - 1.5));
                    let en_rect = egui::Rect::from_min_max(egui::pos2(lang_rect.center().x + 0.5, lang_rect.min.y + 1.5), lang_rect.max - egui::vec2(1.5, 1.5));

                    let tr_resp = ui.interact(tr_rect, ui.scope_id().with("nav-btn-tr"), Sense::CLICK);
                    if tr_resp.clicked() {
                        linc_core::i18n::set_locale(linc_core::i18n::Locale::Tr);
                    }
                    ui.painter().rect_filled(tr_rect, 4, if tr_active { theme::COCKPIT_BLUE } else { Color32::TRANSPARENT });
                    ui.painter().text(
                        tr_rect.center(),
                        Align2::CENTER_CENTER,
                        "TR",
                        FontId::monospace(theme::font_size(9.0)),
                        if tr_active { Color32::WHITE } else { theme::COCKPIT_TEXT_DIM },
                    );

                    let en_resp = ui.interact(en_rect, ui.scope_id().with("nav-btn-en"), Sense::CLICK);
                    if en_resp.clicked() {
                        linc_core::i18n::set_locale(linc_core::i18n::Locale::En);
                    }
                    ui.painter().rect_filled(en_rect, 4, if en_active { theme::COCKPIT_BLUE } else { Color32::TRANSPARENT });
                    ui.painter().text(
                        en_rect.center(),
                        Align2::CENTER_CENTER,
                        "EN",
                        FontId::monospace(theme::font_size(9.0)),
                        if en_active { Color32::WHITE } else { theme::COCKPIT_TEXT_DIM },
                    );

                    // 3. Birleşik Kılavuz, Kısayollar ve G-Code Yardım Menüsü
                    let (menu_rect, menu_response) =
                        ui.allocate_exact_size(Vec2::new(115.0, 26.0), Sense::CLICK);
                    let menu_hovered = menu_response.hovered();
                    ui.painter().rect_filled(
                        menu_rect,
                        6,
                        if menu_hovered { theme::COCKPIT_CARD_HOVER } else { theme::COCKPIT_CARD },
                    );
                    ui.painter().rect_stroke(
                        menu_rect,
                        6,
                        Stroke::new(1.0, if menu_hovered { theme::COCKPIT_BORDER_ACTIVE } else { theme::COCKPIT_BORDER }),
                        StrokeKind::Inside,
                    );
                    // (?) Soru İkonu
                    let q_c = egui::pos2(menu_rect.left() + 13.0, menu_rect.center().y);
                    ui.painter().circle_stroke(q_c, 5.5, Stroke::new(1.2, theme::COCKPIT_CYAN));
                    ui.painter().text(
                        egui::pos2(q_c.x, q_c.y - 0.5),
                        Align2::CENTER_CENTER,
                        "?",
                        FontId::proportional(theme::font_size(9.0)),
                        theme::COCKPIT_CYAN,
                    );
                    let help_label = if tr_active { "YARDIM ▾" } else { "SHORTCUTS ▾" };
                    ui.painter().text(
                        egui::pos2(menu_rect.left() + 24.0, menu_rect.center().y),
                        Align2::LEFT_CENTER,
                        help_label,
                        FontId::proportional(theme::font_size(10.0)),
                        theme::COCKPIT_TEXT_MAIN,
                    );
                    egui::Popup::menu(&menu_response).show(|ui| {
                        ui.set_width(450.0);
                        let tab_id = ui.scope_id().with("help_popup_active_tab");
                        let mut active_tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id).unwrap_or(0));

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📖").size(14.0));
                            let title_text = if tr_active {
                                "KILAVUZ & KISAYOL BAŞVURU MERKEZİ"
                            } else {
                                "HELP & SHORTCUTS REFERENCE"
                            };
                            ui.label(
                                RichText::new(title_text)
                                    .strong()
                                    .size(theme::font_size(11.0))
                                    .color(Color32::WHITE),
                            );
                        });
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            let sc_label = if tr_active { "⌨ Kısayollar" } else { "⌨ Shortcuts" };
                            let sc_btn = ui.selectable_label(active_tab == 0, sc_label);
                            if sc_btn.clicked() {
                                active_tab = 0;
                                ui.data_mut(|d| d.insert_temp(tab_id, active_tab));
                            }

                            let gc_label = if tr_active { "📄 G-Kodu Kılavuzu" } else { "📄 G-Code Guide" };
                            let gc_btn = ui.selectable_label(active_tab == 1, gc_label);
                            if gc_btn.clicked() {
                                active_tab = 1;
                                ui.data_mut(|d| d.insert_temp(tab_id, active_tab));
                            }
                        });
                        ui.separator();

                        egui::ScrollArea::vertical()
                            .id_salt("navigation-help-content")
                            .max_height(400.0)
                            .show(ui, |ui| {
                                if active_tab == 0 {
                                    render_shortcuts_content(ui, &shortcuts, tr_active);
                                } else {
                                    render_gcode_guide_content(ui, &mut open_help_doc, tr_active);
                                }
                            });
                    });

                    // 4. Sistem & Simülasyon Telemetri Kapsülü ([SIM ◨] BaierMill 166 FPS ▾)
                    let clean_name = machine_name
                        .strip_suffix(" Simulation")
                        .unwrap_or(&machine_name);
                    let (sim_rect, sim_resp) =
                        ui.allocate_exact_size(Vec2::new(165.0, 26.0), Sense::CLICK);
                    let sim_hovered = sim_resp.hovered();
                    ui.painter().rect_filled(
                        sim_rect,
                        6,
                        if sim_hovered { theme::COCKPIT_CARD_HOVER } else { theme::COCKPIT_CARD },
                    );
                    ui.painter().rect_stroke(
                        sim_rect,
                        6,
                        Stroke::new(1.0, if sim_hovered { theme::COCKPIT_AMBER } else { theme::COCKPIT_BORDER }),
                        StrokeKind::Inside,
                    );
                    // Amber SIM chip
                    let sim_pill_rect = egui::Rect::from_center_size(
                        egui::pos2(sim_rect.left() + 18.0, sim_rect.center().y),
                        Vec2::new(28.0, 16.0),
                    );
                    let sim_bg = if is_open {
                        theme::COCKPIT_AMBER
                    } else {
                        Color32::from_rgba_unmultiplied(245, 158, 11, 40)
                    };
                    let sim_fg = if is_open {
                        Color32::from_rgb(0x1a, 0x12, 0x04)
                    } else {
                        theme::COCKPIT_AMBER
                    };
                    ui.painter().rect_filled(sim_pill_rect, 4, sim_bg);
                    ui.painter().rect_stroke(sim_pill_rect, 4, Stroke::new(1.0, theme::COCKPIT_AMBER), StrokeKind::Inside);
                    ui.painter().text(
                        sim_pill_rect.center(),
                        Align2::CENTER_CENTER,
                        if is_open { "SIM ◨" } else { "SIM ◧" },
                        FontId::monospace(theme::font_size(8.0)),
                        sim_fg,
                    );
                    // Machine name (no overlap with FPS)
                    ui.painter().text(
                        egui::pos2(sim_rect.left() + 36.0, sim_rect.center().y),
                        Align2::LEFT_CENTER,
                        clean_name,
                        FontId::proportional(theme::font_size(10.0)),
                        Color32::WHITE,
                    );
                    // FPS
                    ui.painter().text(
                        egui::pos2(sim_rect.right() - 14.0, sim_rect.center().y),
                        Align2::RIGHT_CENTER,
                        format!("{:.0} FPS", fps_val),
                        FontId::monospace(theme::font_size(8.5)),
                        theme::COCKPIT_TEXT_DIM,
                    );
                    // Arrow
                    ui.painter().text(
                        egui::pos2(sim_rect.right() - 6.0, sim_rect.center().y),
                        Align2::CENTER_CENTER,
                        "▾",
                        FontId::monospace(theme::font_size(8.0)),
                        theme::COCKPIT_TEXT_DIM,
                    );
                    if sim_resp.clicked() && is_sim {
                        sim_toggle = true;
                    }
                    sim_resp.on_hover_ui(|ui| {
                        ui.label(RichText::new("SİSTEM & SİMÜLASYON TELEMETRİSİ").strong().color(theme::COCKPIT_AMBER));
                        ui.label(format!("Makine Profili: {machine_name}"));
                        ui.label(format!("Render Hızı: {fps_val:.0} FPS"));
                        ui.label("LinuxCNC RT Çekirdek: PREEMPT_RT OK");
                        if is_sim {
                            if is_open {
                                ui.label(RichText::new("• Tıklayarak Simülatör Panelini Gizleyin").size(10.0).color(theme::COCKPIT_TEXT_MUTED));
                            } else {
                                ui.label(RichText::new("• Tıklayarak Simülatör Panelini Açın").size(10.0).color(theme::COCKPIT_BLUE));
                            }
                        }
                    });
                },
            );

        if let Some((index, page)) = nav_action {
            ui.memory_mut(|memory| memory.stop_text_input());
            self.nav.nav_focus = index;
            self.nav.focus_region = FocusRegion::Nav;
            self.request_nav_page(page);
        }
        if open_help_doc {
            self.request_page(Page::Help);
        }
        if sim_toggle {
            self.prefs.preferences.sim_panel_pages.insert(
                self.nav.page.label().to_owned(),
                !is_open,
            );
            self.prefs.dirty = true;
        }
    }
}

fn render_shortcuts_content(
    ui: &mut egui::Ui,
    shortcuts: &[ShortcutGroup],
    tr_active: bool,
) {
    ui.add_space(2.0);

    // 1. Sayfa Özel Dinamik Kısayolları (Key Hints - En başta gösterilir)
    let mut has_page_shortcuts = false;
    for group in shortcuts {
        if !group.items.is_empty() {
            has_page_shortcuts = true;
            if !group.title.is_empty() {
                ui.label(
                    RichText::new(group.title)
                        .size(theme::font_size(9.0))
                        .strong()
                        .color(theme::COCKPIT_CYAN),
                );
            }
            for (key, action) in &group.items {
                render_shortcut_row(ui, key, action);
            }
            ui.add_space(4.0);
        }
    }

    if has_page_shortcuts {
        ui.separator();
        ui.add_space(4.0);
    }

    // 2. Ana Sayfa ve Navigasyon
    ui.label(
        RichText::new(if tr_active {
            "ANA SAYFA VE NAVİGASYON TUŞLARI"
        } else {
            "NAVIGATION & SCREENS"
        })
        .size(theme::font_size(8.5))
        .strong()
        .color(theme::COCKPIT_BLUE),
    );
    let nav_shortcuts = [
        ("F5", if tr_active { "Makine / DRO Ekranı (MAIN)" } else { "Machine / DRO Dashboard" }),
        ("F6", if tr_active { "Program / Dosya Yöneticisi (FILE)" } else { "Program / File Browser" }),
        ("F7", if tr_active { "Takım Tablosu (TOOL)" } else { "Tool Table Management" }),
        ("F8", if tr_active { "İş Parçası Sıfırları & Prob (OFFS)" } else { "Work Offsets & Probe" }),
        ("Ctrl+D", if tr_active { "Teşhis & HAL Sinyalleri (DIAG)" } else { "Diagnostics & HAL Monitor" }),
        ("Ctrl+H", if tr_active { "G-Kodu Dokümantasyonu (HELP)" } else { "G-Code Help & Manual" }),
    ];
    for (key, desc) in nav_shortcuts {
        render_shortcut_row(ui, key, desc);
    }

    ui.add_space(6.0);
    // 3. Makine Kontrol ve Güvenlik
    ui.label(
        RichText::new(if tr_active {
            "MAKİNE KONTROL VE GÜVENLİK TUŞLARI"
        } else {
            "MACHINE CONTROL & SAFETY"
        })
        .size(theme::font_size(8.5))
        .strong()
        .color(theme::COCKPIT_AMBER),
    );
    let safety_shortcuts = [
        ("F1", if tr_active { "Acil Durdurma (E-Stop)" } else { "Emergency Stop (E-Stop)" }),
        ("F2", if tr_active { "Makine Gücü Aç/Kapat (Power On/Off)" } else { "Machine Power On/Off" }),
        ("Boşluk", if tr_active { "Feed Hold (Hareketi Duraklat)" } else { "Feed Hold (Pause)" }),
        ("R / Enter", if tr_active { "G-Kodu Çalıştır / Devam (Start)" } else { "Cycle Start / Resume" }),
        ("ESC", if tr_active { "Hareketi Durdur / İptal (Abort)" } else { "Abort / Cancel Motion" }),
        ("T", if tr_active { "Tek Blok Modu (Single Step)" } else { "Single Step Mode" }),
    ];
    for (key, desc) in safety_shortcuts {
        render_shortcut_row(ui, key, desc);
    }
}

fn render_shortcut_row(ui: &mut egui::Ui, key: &str, desc: &str) {
    ui.horizontal(|ui| {
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(59, 130, 246, 25))
            .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(59, 130, 246, 70)))
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(6, 2))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(key)
                        .monospace()
                        .size(theme::font_size(9.0))
                        .strong()
                        .color(Color32::from_rgb(0x93, 0xc5, 0xfd)),
                );
            });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new(desc)
                    .size(theme::font_size(9.5))
                    .color(theme::COCKPIT_TEXT_MAIN),
            );
        });
    });
}

fn render_gcode_guide_content(ui: &mut egui::Ui, open_full_doc: &mut bool, tr_active: bool) {
    ui.add_space(2.0);
    ui.label(
        RichText::new(if tr_active {
            "STANDART G VE M KODLARI HIZLI KILAVUZU"
        } else {
            "STANDARD G & M CODES REFERENCE"
        })
        .size(theme::font_size(8.5))
        .strong()
        .color(theme::COCKPIT_CYAN),
    );
    ui.add_space(4.0);

    let gcodes = [
        ("G00", if tr_active { "Boşta Hızlı Konumlanma (Rapid Traverse - Kesim yapmadan pozisyon alma)" } else { "Rapid Traverse (Non-cutting rapid positioning)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G01", if tr_active { "Doğrusal Kesme (Linear Feed - Verilen F ilerleme hızında kesim)" } else { "Linear Interpolation (Cutting move at F feedrate)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G02", if tr_active { "Dairesel Kesme CW (Saat Yönünde Dairesel Kavis)" } else { "Circular Interpolation CW (Clockwise arc)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G03", if tr_active { "Dairesel Kesme CCW (Saat Yönünün Tersine Dairesel Kavis)" } else { "Circular Interpolation CCW (Counter-clockwise arc)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G04", if tr_active { "Bekleme Süresi (Dwell - P parametresi kadar duraklama)" } else { "Dwell (Wait for P milliseconds/seconds)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G17-G19", if tr_active { "Çalışma Düzlemi Seçimi (G17: XY Tabla, G18: XZ Torna, G19: YZ)" } else { "Plane Selection (G17: XY, G18: XZ, G19: YZ)" }, Color32::from_rgb(0x93, 0xc5, 0xfd)),
        ("G20/G21", if tr_active { "Birim Seçimi (G20: İnç, G21: Milimetre)" } else { "Units (G20: Inches, G21: Millimeters)" }, Color32::from_rgb(0x93, 0xc5, 0xfd)),
        ("G28/G30", if tr_active { "Referans Noktasına Dönüş (Home / Park Pozisyonu)" } else { "Return to Reference Point (Home/Park)" }, Color32::from_rgb(0x93, 0xc5, 0xfd)),
        ("G54-G59", if tr_active { "İş Parçası Koordinat Sistemleri (WCS 1 - 6 Parça Sıfırları)" } else { "Work Coordinate Systems (WCS 1 to 6 offsets)" }, Color32::from_rgb(0x67, 0xe8, 0xf9)),
        ("G90/G91", if tr_active { "Koordinat Modu (G90: Mutlak / Absolute, G91: Artımlı / Incremental)" } else { "Distance Mode (G90: Absolute, G91: Incremental)" }, Color32::from_rgb(0x93, 0xc5, 0xfd)),
        ("G94/G95", if tr_active { "İlerleme Modu (G94: mm/dk, G95: mm/devir torna)" } else { "Feed Mode (G94: Units/min, G95: Units/rev)" }, Color32::from_rgb(0x93, 0xc5, 0xfd)),
        ("M03", if tr_active { "Spindle Milini Saat Yönünde Döndür (CW) [Örn: M3 S18000]" } else { "Spindle CW On [e.g. M3 S18000]" }, Color32::from_rgb(0xfc, 0xd3, 0x4d)),
        ("M04", if tr_active { "Spindle Milini Ters Saat Yönünde Döndür (CCW)" } else { "Spindle CCW On" }, Color32::from_rgb(0xfc, 0xd3, 0x4d)),
        ("M05", if tr_active { "Spindle Milini Durdur (Spindle Stop)" } else { "Spindle Stop" }, Color32::from_rgb(0xfc, 0xd3, 0x4d)),
        ("M06", if tr_active { "Takım Değiştirme Komutu (Tool Change - Örn: T1 M6)" } else { "Tool Change (e.g. T1 M6)" }, Color32::from_rgb(0x86, 0xef, 0xac)),
        ("M08/M09", if tr_active { "Soğutma Sıvısı (M8: Soğutmayı Aç, M9: Soğutmayı Kapat)" } else { "Coolant (M8: Flood Coolant On, M9: Coolant Off)" }, Color32::from_rgb(0x86, 0xef, 0xac)),
        ("M30", if tr_active { "Program Sonu ve Başa Dönüş (End of Program & Rewind)" } else { "Program End and Rewind" }, Color32::from_rgb(0xf8, 0x71, 0x71)),
    ];

    for (code, desc, color) in gcodes {
        ui.horizontal(|ui| {
            egui::Frame::new()
                .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 12))
                .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30)))
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(6, 2))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(code)
                            .monospace()
                            .size(theme::font_size(9.5))
                            .strong()
                            .color(color),
                    );
                });
            ui.label(
                RichText::new(desc)
                    .size(theme::font_size(9.5))
                    .color(theme::COCKPIT_TEXT_MAIN),
            );
        });
        ui.add_space(2.0);
    }

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);
    let open_full_btn = ui.add(
        egui::Button::new(
            RichText::new(if tr_active {
                "📖 DETAYLI G-KODU DOKÜMANTASYONUNU AÇ (Ctrl+H)"
            } else {
                "📖 OPEN FULL G-CODE DOCUMENTATION (Ctrl+H)"
            })
            .strong()
            .size(theme::font_size(10.0))
            .color(Color32::WHITE),
        )
        .fill(theme::COCKPIT_BLUE)
        .corner_radius(6)
        .min_size(Vec2::new(ui.available_width(), 28.0)),
    );
    if open_full_btn.clicked() {
        *open_full_doc = true;
        ui.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub(super) fn pages_follow_physical_mode() {
        // MDI opens MAIN, whose program card the mode swaps for the pane.
        assert_eq!(
            mode_page(&PanelModeValidity::Valid(PanelMode::Mdi)),
            Some(Page::Machine)
        );
        assert_eq!(
            mode_page(&PanelModeValidity::Valid(PanelMode::Home)),
            Some(Page::Homing)
        );
        assert_eq!(
            mode_page(&PanelModeValidity::Valid(PanelMode::Jog)),
            Some(Page::Manual)
        );
        assert_eq!(
            mode_page(&PanelModeValidity::Invalid(vec![
                PanelMode::Mdi,
                PanelMode::Auto
            ])),
            None
        );
    }
}

/// Page routing state: which page is open, where it returns to, and which
/// navigation row holds the ring.
pub(crate) struct NavUi {
    pub page: Page,
    pub last_non_mode_page: Page,
    pub files_return_page: Page,
    pub help_return_page: Page,
    /// Where Esc on the Diagnostics page goes back to: the page the operator
    /// left to look at it, not the program reset.
    pub diagnostics_return_page: Page,
    pub nav_focus: usize,
    pub focus_region: FocusRegion,
    pub last_mode: Option<PanelModeValidity>,
    pub homing_return_page: Page,
    pub page_focus_requested: bool,
    /// Navigation entries the profile shows, in display order.
    pub pages: Vec<Page>,
}
