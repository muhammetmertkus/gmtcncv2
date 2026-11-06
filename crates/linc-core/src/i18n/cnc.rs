//! Runtime LinuxCNC error and operator message translations.
//!
//! Aligned with `gmtcnc-main/src/i18n/cncTranslate.ts` and `cncCatalogs/tr.ts`.
//! Translates NML error and operator messages from LinuxCNC into Turkish.
//! Preserves G/M codes, numbers, axis names, and file paths.
//! Falls back to the original English text for unmatched messages.

use std::borrow::Cow;

/// Curated pattern-based CNC operator messages for LinuxCNC runtime events.
pub static CNC_TR_PATTERNS: &[(&str, &str)] = &[
    (
        "command (%s) cannot be executed until the machine is out of E-stop and turned on",
        "komut (%s), makine E-stop'tan çıkıp açılana kadar yürütülemez",
    ),
    (
        "can't do that (%s:%d) in manual mode",
        "bunu (%s:%d) manuel modda yapamazsınız",
    ),
    (
        "can't do that (%s) in manual mode",
        "bunu (%s) manuel modda yapamazsınız",
    ),
    (
        "can't do that (%s) in auto mode with the interpreter idle",
        "bunu (%s) otomatik modda interpreter boştayken yapamazsınız",
    ),
    (
        "can't do that (%s) in auto mode with the interpreter reading",
        "bunu (%s) otomatik modda interpreter okurken yapamazsınız",
    ),
    (
        "can't do that (%s) in auto mode with the interpreter paused",
        "bunu (%s) otomatik modda interpreter duraklatılmışken yapamazsınız",
    ),
    (
        "can't do that (%s) in auto mode with the interpreter waiting",
        "bunu (%s) otomatik modda interpreter beklerken yapamazsınız",
    ),
    (
        "can't do that (%s:%d) in MDI mode",
        "bunu (%s:%d) MDI modunda yapamazsınız",
    ),
    (
        "can't do that (%s) in MDI mode",
        "bunu (%s) MDI modunda yapamazsınız",
    ),
    (
        "Can't switch mode while mode is AUTO and interpreter is not IDLE",
        "Mod AUTO iken ve interpreter BOŞTA değilken mod değiştirilemez",
    ),
    (
        "Can't issue MDI command when not homed",
        "Home yapılmadan MDI komutu verilemez",
    ),
    (
        "Must be in MDI mode to issue MDI command",
        "MDI komutu vermek için MDI modunda olunmalıdır",
    ),
    (
        "Can't run a program when not homed",
        "Home yapılmadan program çalıştırılamaz",
    ),
    (
        "all joints must be homed before going into teleop mode",
        "Teleop moduna geçmeden önce tüm eksenler Home yapılmalıdır",
    ),
    (
        "linear and rotary motion together not supported",
        "Doğrusal ve döner hareketin birlikte kullanımı desteklenmiyor",
    ),
    (
        "Cannot unhome while moving",
        "Hareket halindeyken Home iptal edilemez (unhome)",
    ),
    (
        "Exceeded positive soft limit on joint %s",
        "Eksen %s pozitif strok limiti aşıldı",
    ),
    (
        "Exceeded negative soft limit on joint %s",
        "Eksen %s negatif strok limiti aşıldı",
    ),
    (
        "Exceeded positive soft limit",
        "Pozitif strok limiti aşıldı",
    ),
    (
        "Exceeded negative soft limit",
        "Negatif strok limiti aşıldı",
    ),
    (
        "Spindle speed out of range",
        "Spindle devri izin verilen aralık dışında",
    ),
    (
        "Spindle not turning",
        "Spindle dönmüyor",
    ),
    (
        "Feed rate is zero",
        "Feed (ilerleme hızı) sıfır",
    ),
    (
        "G4 not allowed with G2 or G3 because they both use P",
        "G4 ile G2/G3 birlikte kullanılamaz (ikisi de P parametresini kullanır)",
    ),
    (
        "Linear move on line %s would exceed positive soft limit on joint %s",
        "Satır %s üzerindeki doğrusal hareket eksen %s pozitif strok limitini aşıyor",
    ),
    (
        "Linear move on line %s would exceed negative soft limit on joint %s",
        "Satır %s üzerindeki doğrusal hareket eksen %s negatif strok limitini aşıyor",
    ),
    (
        "Near line %s: %s",
        "Satır %s civarı: %s",
    ),
    (
        "can't open %s",
        "Dosya açılamıyor: %s",
    ),
    (
        "failed to close file",
        "Dosya kapatılamadı",
    ),
];

/// Match an English message against a pattern template and, if matched, substitute
/// extracted values into the target Turkish template.
pub fn match_and_translate<'a>(msg: &'a str) -> Option<Cow<'a, str>> {
    let trimmed_msg = msg.trim_end_matches(['.', '!', ' ']);

    for &(src_pattern, tr_template) in CNC_TR_PATTERNS {
        // Direct static match
        if msg == src_pattern || trimmed_msg == src_pattern.trim_end_matches(['.', '!', ' ']) {
            return Some(Cow::Borrowed(tr_template));
        }

        // Pattern matching with placeholders
        if src_pattern.contains('%') {
            if let Some(translated) = try_pattern_substitute(msg, src_pattern, tr_template) {
                return Some(Cow::Owned(translated));
            }
        }
    }

    None
}

/// Helper to match variable parts between format specifiers and reconstruct the target string.
fn try_pattern_substitute(msg: &str, src_pattern: &str, tr_template: &str) -> Option<String> {
    // Split pattern around %s / %d / %c / %f
    let mut parts: Vec<&str> = Vec::new();
    let mut last = 0;
    let bytes = src_pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 1 < bytes.len() {
            let next = bytes[i + 1];
            if matches!(next, b's' | b'd' | b'c' | b'f') {
                parts.push(&src_pattern[last..i]);
                i += 2;
                last = i;
                continue;
            }
        }
        i += 1;
    }
    parts.push(&src_pattern[last..]);

    if parts.len() < 2 {
        return None;
    }

    // Match parts in msg
    let mut remainder = msg;
    let mut captured = Vec::new();

    // Check first prefix
    let prefix = parts[0];
    if !remainder.starts_with(prefix) {
        return None;
    }
    remainder = &remainder[prefix.len()..];

    for idx in 1..parts.len() {
        let separator = parts[idx];
        if separator.is_empty() {
            // Last part is empty, remainder is the last token
            captured.push(remainder);
            remainder = "";
        } else {
            let pos = remainder.find(separator)?;
            captured.push(&remainder[..pos]);
            remainder = &remainder[pos + separator.len()..];
        }
    }

    // Substitute captured into tr_template
    let mut result = String::with_capacity(tr_template.len() + 32);
    let mut tr_idx = 0;
    let tr_bytes = tr_template.as_bytes();
    let mut cap_iter = captured.into_iter();

    let mut j = 0;
    while j < tr_bytes.len() {
        if tr_bytes[j] == b'%' && j + 1 < tr_bytes.len() {
            let next = tr_bytes[j + 1];
            if matches!(next, b's' | b'd' | b'c' | b'f') {
                result.push_str(&tr_template[tr_idx..j]);
                if let Some(val) = cap_iter.next() {
                    result.push_str(val);
                }
                j += 2;
                tr_idx = j;
                continue;
            }
        }
        j += 1;
    }
    result.push_str(&tr_template[tr_idx..]);

    Some(result)
}
