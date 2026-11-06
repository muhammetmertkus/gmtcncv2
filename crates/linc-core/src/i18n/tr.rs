//! Turkish (tr) message catalog for LiNC.
//!
//! Aligned with `gmtcnc-main/src/i18n/locales/tr/viewport.ts` and `shell.ts`.
//! Default built-in locale for GMTCNT CNC controllers.
//! Formulated according to shop-floor operator terminology in Turkey.

pub static TR_MESSAGES: &[(&str, &str)] = &[
    // Viewport presets & camera
    ("viewport.presets.top", "Üstten görünüm"),
    ("viewport.presets.bottom", "Alttan görünüm"),
    ("viewport.presets.front", "Önden görünüm"),
    ("viewport.presets.back", "Arkadan görünüm"),
    ("viewport.presets.right", "Sağdan görünüm"),
    ("viewport.presets.left", "Soldan görünüm"),
    ("viewport.presets.iso", "İzometrik görünüm"),
    ("viewport.presets.free", "Özel görünüm"),
    ("viewport.viewMenu.ariaLabel", "Kamera görünümü"),
    ("viewport.viewMenu.headingStandard", "Standart görünümler"),
    ("viewport.viewMenu.headingActions", "Eylemler"),
    ("viewport.viewMenu.fit", "Modele sığdır"),
    ("viewport.viewMenu.reset", "Görünümü sıfırla"),

    // 3D ViewCube faces
    ("viewport.cube.top", "ÜST"),
    ("viewport.cube.bottom", "ALT"),
    ("viewport.cube.front", "ÖN"),
    ("viewport.cube.back", "ARKA"),
    ("viewport.cube.right", "SAĞ"),
    ("viewport.cube.left", "SOL"),

    // Quick snap pills below ViewCube
    ("viewport.snap.iso", "İZO"),
    ("viewport.snap.top", "ÜST"),
    ("viewport.snap.front", "ÖN"),
    ("viewport.snap.right", "SAĞ"),

    // 3D Viewport Legend & Header
    ("viewport.legend.toolpath", "TAKIM YOLU"),
    ("viewport.legend.cut", "KESİM"),
    ("viewport.legend.rapid", "RAPID"),
    ("viewport.legend.live", "CANLI"),
    ("viewport.legend.tool", "TAKIM"),
    ("viewport.legend.stock", "HAM PARÇA"),
    ("viewport.legend.remaining", "KALAN YOL"),
    ("viewport.legend.program_cut", "PROGRAM KESİM"),
    ("viewport.legend.live_toolpath", "CANLI TAKIM YOLU"),
    ("viewport.legend.drag_orbit", "ÇEVİRMEK İÇİN SÜRÜKLEYİN"),

    // Viewport Status Readouts
    ("viewport.status.no_program", "PROGRAM YÜKLÜ DEĞİL"),
    ("viewport.status.no_program_tooltip", "İşlenecek G-kodu programı yüklü değil"),
    ("viewport.status.program_tooltip", "Yüklü G-kodu program dosyası"),

    // DRO HUD Labels
    ("viewport.dro.vel", "Anlık Hız"),
    ("viewport.dro.feed", "Feed"),
    ("viewport.dro.est", "Tahmini Süre"),
    ("viewport.dro.cut", "Kesim"),
    ("viewport.dro.rap", "Rapid"),
    ("viewport.dro.dtg", "DTG"),
    ("viewport.dro.tlo", "TLO"),
    ("viewport.dro.line", "SATIR"),
    ("viewport.dro.elapsed", "Geçen Süre"),
    ("viewport.dro.remaining", "Kalan Süre"),
    ("viewport.dro.rpm", "Devir (RPM)"),

    // Viewport Visibility Toggles (LinuxCNC AXIS Parity)
    ("viewport.toggles.rapids", "RAPID"),
    ("viewport.toggles.rapids_short", "RAP"),
    ("viewport.toggles.rapids_tooltip", "G0 Rapid (boşta pozisyonlama) hareket çizgilerini aç/kapat"),
    ("viewport.toggles.tool", "TAKIM"),
    ("viewport.toggles.tool_short", "TKM"),
    ("viewport.toggles.tool_tooltip", "3D Takım ve Spindle modelini aç/kapat"),
    ("viewport.toggles.stock", "HAM PARÇA"),
    ("viewport.toggles.stock_short", "HAM"),
    ("viewport.toggles.stock_tooltip", "Ham parça (stok) zarfını aç/kapat"),
    ("viewport.toggles.limits", "STROK"),
    ("viewport.toggles.limits_short", "STR"),
    ("viewport.toggles.limits_tooltip", "Eksen strok limitleri sınır kutusunu aç/kapat"),
    ("viewport.toggles.plane", "DÜZLEM"),
    ("viewport.toggles.plane_short", "DÜZ"),
    ("viewport.toggles.plane_tooltip", "Aktif çalışma düzlemi (G17/G18/G19) ızgarasını aç/kapat"),

    // Parametric Tool Cutter Types
    ("viewport.tools.endmill", "Parmak Freze"),
    ("viewport.tools.ballend", "Küre Freze"),
    ("viewport.tools.chamfer", "Pah Frezesi"),
    ("viewport.tools.drill", "Matkap"),
    ("viewport.tools.probe", "3D Prob"),
    ("viewport.tools.torus", "Köşe Radyüslü Freze"),
    ("viewport.tools.lathe", "Torna Kateri"),
    ("viewport.tools.dovetail", "Kırlangıç Freze"),
    ("viewport.tools.reamer", "Rayba"),
    ("viewport.tools.slittingsaw", "Kanal Testeresi"),
    ("viewport.tools.threadmill", "Diş Frezesi"),
    ("viewport.tools.vbit", "V-Kanal Çakısı"),

    // Workpiece Stock Models
    ("viewport.workpiece.box", "Kutu Ham Parça"),
    ("viewport.workpiece.cylinder", "Silindir Ham Parça"),
    ("viewport.workpiece.tube", "Boru Ham Parça"),

    // G-Code Properties Dialog / Summary
    ("viewport.properties.title", "G-Kodu Özellikleri"),
    ("viewport.properties.file_name", "Dosya Adı"),
    ("viewport.properties.file_size", "Dosya Boyutu"),
    ("viewport.properties.line_count", "G-Kodu Satırı"),
    ("viewport.properties.cut_dist", "Kesim Mesafesi"),
    ("viewport.properties.rapid_dist", "Rapid Mesafesi"),
    ("viewport.properties.est_time", "Tahmini Süre"),
    ("viewport.properties.extents", "Boyutlar (X / Y / Z)"),
    ("viewport.properties.tool_list", "Kullanılan Takımlar"),

    // Shell tabs & navigation
    ("shell.tabs.run", "ÇALIŞTIR"),
    ("shell.tabs.mdi", "MDI"),
    ("shell.tabs.jog", "JOG"),
    ("shell.tabs.offsets", "SIFIRLAMA"),
    ("shell.tabs.diag", "TEŞHİS"),

    // Status Bar Badges (replaces legacy full-width banners)
    ("shell.badges.estop", "ESTOP"),
    ("shell.badges.estop_tooltip", "Acil Stop Devrede — Tüm hareketler ve makine eylemleri kilitlendi"),
    ("shell.badges.limits_override", "LİMİT OVR"),
    ("shell.badges.limits_override_tooltip", "Limit Baypas Aktif — Eksen sınır anahtarları yok sayılıyor! Mekanik çarpma riski"),
    ("shell.badges.probe_trip", "PROB TEMAS"),
    ("shell.badges.probe_trip_tooltip", "3D Prob Temas Hatası — Hareket kilitlendi. Sıfırlamak için tıklayın"),
    ("shell.badges.panel_offline", "PANEL KOPTU"),
    ("shell.badges.panel_offline_tooltip", "Kumanda Paneli Bağlantısı Koptu — Makine eylemleri engellendi"),
    ("shell.badges.panel_invalid", "PANEL MOD"),
    ("shell.badges.panel_invalid_tooltip", "Panel Modu Geçersiz — Emniyet amaçlı kilitlendi"),
    ("shell.badges.protect", "KİLİT"),
    ("shell.badges.protect_tooltip", "Program Kilidi Açık — Düzenleme, ofset yazımı ve veri girişleri engellendi"),
    ("shell.badges.ext_change", "DIŞ DEĞİŞİM"),
    ("shell.badges.ext_change_tooltip", "Program Harici Değişti — Kaydedilmemiş tampon korundu"),
];
