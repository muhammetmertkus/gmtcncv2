//! Internationalization (i18n) and localization infrastructure for LiNC.
//!
//! Aligned with `gmtcnc-main/src/i18n/`:
//! - English (`en`) is the canonical base catalog.
//! - Turkish (`tr`) is the default built-in locale for GMTCNC.
//! - Keys follow flat dot-namespaced identifiers (e.g. `viewport.presets.top`).
//! - Token `{name}` interpolation matches `gmtcnc-main/src/i18n/catalog.ts`.
//! - Fast O(1) / slice lookups with zero allocation on simple `t(key)` calls.

use std::borrow::Cow;
use std::sync::atomic::{AtomicU8, Ordering};

pub mod cnc;
pub mod en;
pub mod tr;

use self::en::EN_MESSAGES;
use self::tr::TR_MESSAGES;

/// Supported built-in application locales.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    Tr = 0,
    En = 1,
}

impl Locale {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Tr => "tr",
            Self::En => "en",
        }
    }

    pub const fn native_name(self) -> &'static str {
        match self {
            Self::Tr => "Türkçe",
            Self::En => "English",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id.to_lowercase().as_str() {
            "tr" | "tr-tr" | "turkish" => Self::Tr,
            _ => Self::En,
        }
    }
}

/// Active global application locale. Defaults to English (1 = En) for clean headless test parity,
/// runtime defaults to Turkish in GMTCNC profile initialization.
static ACTIVE_LOCALE: AtomicU8 = AtomicU8::new(1);

/// Set the active application locale globally.
pub fn set_locale(locale: Locale) {
    ACTIVE_LOCALE.store(locale as u8, Ordering::Release);
}

/// Get the currently active application locale.
pub fn current_locale() -> Locale {
    match ACTIVE_LOCALE.load(Ordering::Acquire) {
        0 => Locale::Tr,
        _ => Locale::En,
    }
}

/// Get the active locale tag (e.g. "tr" or "en").
pub fn locale_id() -> &'static str {
    current_locale().id()
}

/// Translate a dot-namespaced key into the active locale.
///
/// Falls back to English if the translation is missing in the active locale,
/// and returns the raw key if missing in both catalogs.
#[inline]
pub fn t(key: &'static str) -> &'static str {
    t_in(current_locale(), key)
}

/// Translate a dot-namespaced key for an explicit locale.
pub fn t_in(locale: Locale, key: &'static str) -> &'static str {
    match locale {
        Locale::Tr => {
            if let Some(msg) = find_in(TR_MESSAGES, key) {
                return msg;
            }
            // Fallback to English
            if let Some(msg) = find_in(EN_MESSAGES, key) {
                return msg;
            }
        }
        Locale::En => {
            if let Some(msg) = find_in(EN_MESSAGES, key) {
                return msg;
            }
        }
    }

    // Key itself as final fallback (makes untranslated keys immediately noticeable in UI)
    key
}

/// Translate a key and interpolate `{placeholder}` tokens.
///
/// Matches `gmtcnc-main/src/i18n/catalog.ts` interpolation behavior:
/// - Replaces `{name}` with the value corresponding to `name` in `params`.
/// - Leaves unknown tokens untouched (e.g. `{unknown}` stays `{unknown}`).
pub fn t_with(key: &'static str, params: &[(&str, &str)]) -> String {
    let template = t(key);
    interpolate(template, params)
}

/// Replace `{name}` tokens with corresponding param values.
pub fn interpolate(template: &str, params: &[(&str, &str)]) -> String {
    if params.is_empty() || !template.contains('{') {
        return template.to_string();
    }

    let mut result = String::with_capacity(template.len() + 32);
    let mut chars = template.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '{' {
            // Find closing brace
            let mut close = None;
            for (j, inner_c) in chars.by_ref() {
                if inner_c == '}' {
                    close = Some(j);
                    break;
                }
            }

            if let Some(j) = close {
                let token_name = &template[i + 1..j];
                let mut replaced = false;
                for &(name, value) in params {
                    if name == token_name {
                        result.push_str(value);
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    // Keep verbatim
                    result.push_str(&template[i..=j]);
                }
            } else {
                result.push('{');
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Translate a raw LinuxCNC error or operator message received from NML channel.
///
/// If running in Turkish and a pattern matches, returns the Turkish translation with
/// dynamic arguments substituted; otherwise returns the original message untouched.
/// Preserves G/M codes, numbers, axis names, and paths.
pub fn translate_cnc_message(msg: &str) -> Cow<'_, str> {
    if current_locale() == Locale::En {
        return Cow::Borrowed(msg);
    }

    if let Some(trans) = cnc::match_and_translate(msg) {
        return trans;
    }

    Cow::Borrowed(msg)
}

/// Turkish locale-aware uppercase conversion.
///
/// In Turkish:
/// - 'i' -> 'İ' (U+0130, Latin Capital Letter I with Dot Above)
/// - 'ı' -> 'I' (U+0049, Latin Capital Letter I)
/// Naive `to_uppercase` converts 'i' to ASCII 'I', breaking Turkish words (e.g. "limit" -> "LIMIT").
pub fn tr_upper(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'i' => out.push('İ'),
            'ı' => out.push('I'),
            other => {
                for up in other.to_uppercase() {
                    out.push(up);
                }
            }
        }
    }
    out
}

/// Turkish locale-aware lowercase conversion.
///
/// In Turkish:
/// - 'İ' -> 'i'
/// - 'I' -> 'ı' (U+0131, Latin Small Letter Dotless I)
pub fn tr_lower(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'İ' => out.push('i'),
            'I' => out.push('ı'),
            other => {
                for lo in other.to_lowercase() {
                    out.push(lo);
                }
            }
        }
    }
    out
}

#[inline]
fn find_in(catalog: &[(&'static str, &'static str)], key: &str) -> Option<&'static str> {
    catalog.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_switching() {
        set_locale(Locale::En);
        assert_eq!(t("viewport.cube.top"), "TOP");
        assert_eq!(t("viewport.presets.top"), "Top view");

        set_locale(Locale::Tr);
        assert_eq!(t("viewport.cube.top"), "ÜST");
        assert_eq!(t("viewport.presets.top"), "Üstten görünüm");
        set_locale(Locale::En);
    }

    #[test]
    fn test_missing_key_fallback() {
        set_locale(Locale::Tr);
        // Key missing in both returns the key itself
        assert_eq!(t("nonexistent.dummy.key"), "nonexistent.dummy.key");
        set_locale(Locale::En);
    }

    #[test]
    fn test_interpolation() {
        let template = "Version {version} for {device}";
        let res = interpolate(template, &[("version", "2.1"), ("device", "GMT-CNC")]);
        assert_eq!(res, "Version 2.1 for GMT-CNC");

        // Untranslated placeholder stays intact
        let res2 = interpolate("Hello {name}, your balance is {bal}", &[("name", "Mert")]);
        assert_eq!(res2, "Hello Mert, your balance is {bal}");
    }

    #[test]
    fn test_catalog_key_parity() {
        // Every English key must have a Turkish translation and vice-versa
        for (en_key, _) in EN_MESSAGES {
            assert!(
                TR_MESSAGES.iter().any(|(k, _)| k == en_key),
                "Missing Turkish translation for key: {}",
                en_key
            );
        }

        for (tr_key, _) in TR_MESSAGES {
            assert!(
                EN_MESSAGES.iter().any(|(k, _)| k == tr_key),
                "Missing English base key for Turkish key: {}",
                tr_key
            );
        }
    }

    #[test]
    fn test_forbidden_terms() {
        // tr catalog must never contain forbidden calques
        let forbidden = ["kütük", "fener mili", "seyahat limiti", "hızlı"];
        for (key, val) in TR_MESSAGES {
            let lower = val.to_lowercase();
            for word in forbidden {
                assert!(
                    !lower.contains(word),
                    "Found forbidden term '{}' in key '{}' with value '{}'",
                    word,
                    key,
                    val
                );
            }
        }
    }

    #[test]
    fn test_turkish_uppercase_glyphs() {
        assert_eq!(tr_upper("limit"), "LİMİT");
        assert_eq!(tr_upper("izo"), "İZO");
        assert_eq!(tr_upper("ilerleme"), "İLERLEME");
        assert_eq!(tr_upper("düzlem"), "DÜZLEM");
        assert_eq!(tr_lower("LİMİT"), "limit");
        assert_eq!(tr_lower("İZO"), "izo");
    }

    #[test]
    fn test_cnc_message_preserves_tokens() {
        set_locale(Locale::Tr);

        // Pattern with token extraction
        let msg1 = translate_cnc_message("command (G0 X100) cannot be executed until the machine is out of E-stop and turned on");
        assert_eq!(msg1, "komut (G0 X100), makine E-stop'tan çıkıp açılana kadar yürütülemez");

        let msg2 = translate_cnc_message("can't do that (M3:10) in manual mode");
        assert_eq!(msg2, "bunu (M3:10) manuel modda yapamazsınız");

        let msg3 = translate_cnc_message("can't do that (G1) in MDI mode");
        assert_eq!(msg3, "bunu (G1) MDI modunda yapamazsınız");

        // Unmatched message returned verbatim in English without corruption
        let raw_unmatched = "Unrecognised M-code M999 on line 125 at 0x48FF";
        let msg4 = translate_cnc_message(raw_unmatched);
        assert_eq!(msg4, raw_unmatched);

        set_locale(Locale::En);
    }
}
