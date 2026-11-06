//! Canonical English (en) message catalog for LiNC.
//!
//! Aligned with `gmtcnc-main/src/i18n/locales/en/viewport.ts` and `shell.ts`.
//! Every valid message key must exist in this catalog as the base fallback.

pub static EN_MESSAGES: &[(&str, &str)] = &[
    // Viewport presets & camera
    ("viewport.presets.top", "Top view"),
    ("viewport.presets.bottom", "Bottom view"),
    ("viewport.presets.front", "Front view"),
    ("viewport.presets.back", "Back view"),
    ("viewport.presets.right", "Right view"),
    ("viewport.presets.left", "Left view"),
    ("viewport.presets.iso", "Isometric view"),
    ("viewport.presets.free", "Custom view"),
    ("viewport.viewMenu.ariaLabel", "Camera view"),
    ("viewport.viewMenu.headingStandard", "Standard views"),
    ("viewport.viewMenu.headingActions", "Actions"),
    ("viewport.viewMenu.fit", "Fit to model"),
    ("viewport.viewMenu.reset", "Reset view"),

    // 3D ViewCube faces
    ("viewport.cube.top", "TOP"),
    ("viewport.cube.bottom", "BOTTOM"),
    ("viewport.cube.front", "FRONT"),
    ("viewport.cube.back", "BACK"),
    ("viewport.cube.right", "RIGHT"),
    ("viewport.cube.left", "LEFT"),

    // Quick snap pills below ViewCube
    ("viewport.snap.iso", "ISO"),
    ("viewport.snap.top", "TOP"),
    ("viewport.snap.front", "FRT"),
    ("viewport.snap.right", "RGT"),

    // 3D Viewport Legend & Header
    ("viewport.legend.toolpath", "TOOLPATH"),
    ("viewport.legend.cut", "CUT"),
    ("viewport.legend.rapid", "RAPID"),
    ("viewport.legend.live", "LIVE"),
    ("viewport.legend.tool", "TOOL"),
    ("viewport.legend.stock", "STOCK"),
    ("viewport.legend.remaining", "REMAINING"),
    ("viewport.legend.program_cut", "PROGRAM CUT"),
    ("viewport.legend.live_toolpath", "LIVE TOOLPATH"),
    ("viewport.legend.drag_orbit", "DRAG TO ORBIT"),

    // Viewport Status Readouts
    ("viewport.status.no_program", "NO PROGRAM LOADED"),
    ("viewport.status.no_program_tooltip", "No G-code program loaded"),
    ("viewport.status.program_tooltip", "Loaded G-code program file"),

    // DRO HUD Labels
    ("viewport.dro.vel", "Vel"),
    ("viewport.dro.feed", "Feed"),
    ("viewport.dro.est", "Est"),
    ("viewport.dro.cut", "Cut"),
    ("viewport.dro.rap", "Rap"),
    ("viewport.dro.dtg", "DTG"),
    ("viewport.dro.tlo", "TLO"),
    ("viewport.dro.line", "LN"),
    ("viewport.dro.elapsed", "Elapsed"),
    ("viewport.dro.remaining", "Remaining"),
    ("viewport.dro.rpm", "RPM"),

    // Viewport Visibility Toggles (LinuxCNC AXIS Parity)
    ("viewport.toggles.rapids", "RAPID"),
    ("viewport.toggles.rapids_short", "RAP"),
    ("viewport.toggles.rapids_tooltip", "Toggle rapid traverse lines (G0)"),
    ("viewport.toggles.tool", "TOOL"),
    ("viewport.toggles.tool_short", "TOOL"),
    ("viewport.toggles.tool_tooltip", "Toggle 3D cutter toolhead"),
    ("viewport.toggles.stock", "STOCK"),
    ("viewport.toggles.stock_short", "STK"),
    ("viewport.toggles.stock_tooltip", "Toggle workpiece stock bounding envelope"),
    ("viewport.toggles.limits", "LIMIT"),
    ("viewport.toggles.limits_short", "LIM"),
    ("viewport.toggles.limits_tooltip", "Toggle machine travel limits boundary box"),
    ("viewport.toggles.plane", "PLANE"),
    ("viewport.toggles.plane_short", "PLN"),
    ("viewport.toggles.plane_tooltip", "Toggle active working plane grid (G17/G18/G19)"),

    // Parametric Tool Cutter Types
    ("viewport.tools.endmill", "Endmill"),
    ("viewport.tools.ballend", "Ballend"),
    ("viewport.tools.chamfer", "Chamfer"),
    ("viewport.tools.drill", "Drill"),
    ("viewport.tools.probe", "Probe"),
    ("viewport.tools.torus", "Torus"),
    ("viewport.tools.lathe", "Lathe"),
    ("viewport.tools.dovetail", "Dovetail"),
    ("viewport.tools.reamer", "Reamer"),
    ("viewport.tools.slittingsaw", "Slitting Saw"),
    ("viewport.tools.threadmill", "Thread Mill"),
    ("viewport.tools.vbit", "V-Bit"),

    // Workpiece Stock Models
    ("viewport.workpiece.box", "Box Stock"),
    ("viewport.workpiece.cylinder", "Cylinder Stock"),
    ("viewport.workpiece.tube", "Tube Stock"),

    // G-Code Properties Dialog / Summary
    ("viewport.properties.title", "G-Code Properties"),
    ("viewport.properties.file_name", "File Name"),
    ("viewport.properties.file_size", "File Size"),
    ("viewport.properties.line_count", "G-Code Lines"),
    ("viewport.properties.cut_dist", "Cutting Distance"),
    ("viewport.properties.rapid_dist", "Rapid Distance"),
    ("viewport.properties.est_time", "Estimated Time"),
    ("viewport.properties.extents", "Extents (X / Y / Z)"),
    ("viewport.properties.tool_list", "Tools Used"),

    // Shell tabs & navigation
    ("shell.tabs.run", "RUN"),
    ("shell.tabs.mdi", "MDI"),
    ("shell.tabs.jog", "JOG"),
    ("shell.tabs.offsets", "OFFSETS"),
    ("shell.tabs.diag", "DIAG"),

    // Status Bar Badges (replaces legacy full-width banners)
    ("shell.badges.estop", "ESTOP"),
    ("shell.badges.estop_tooltip", "Emergency Stop Active — Motion and machine actions blocked"),
    ("shell.badges.limits_override", "LIMIT OVR"),
    ("shell.badges.limits_override_tooltip", "Limits Override Active — Limit switches are ignored! Risk of machine collision"),
    ("shell.badges.probe_trip", "PROBE TRIP"),
    ("shell.badges.probe_trip_tooltip", "3D Probe Tripped — Motion blocked. Click to reset"),
    ("shell.badges.panel_offline", "PANEL OFF"),
    ("shell.badges.panel_offline_tooltip", "Operator Panel Offline — Machine actions blocked"),
    ("shell.badges.panel_invalid", "PANEL MODE"),
    ("shell.badges.panel_invalid_tooltip", "Panel Mode Invalid — Fails closed for safety"),
    ("shell.badges.protect", "PROTECT"),
    ("shell.badges.protect_tooltip", "Program Protect On — Editing, writes and offsets locked"),
    ("shell.badges.ext_change", "EXT CHANGE"),
    ("shell.badges.ext_change_tooltip", "Program Changed Externally — Unsaved buffer preserved"),
];
