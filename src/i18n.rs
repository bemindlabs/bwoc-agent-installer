/// i18n.rs — minimal bilingual support (English default, Thai alternate).
///
/// `Lang` is threaded through `App` and all rendering calls.  Nothing here
/// uses global state — callers pass `lang` explicitly.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    En,
    Th,
}

impl Lang {
    /// Toggle between the two languages.
    pub fn toggle(self) -> Lang {
        match self {
            Lang::En => Lang::Th,
            Lang::Th => Lang::En,
        }
    }

    /// Parse from a short string (`"en"` / `"th"`).  Returns `None` on
    /// unrecognised input so the caller can fall back to the default.
    pub fn from_str(s: &str) -> Option<Lang> {
        match s.to_ascii_lowercase().as_str() {
            "en" => Some(Lang::En),
            "th" => Some(Lang::Th),
            _ => None,
        }
    }
}

/// A bilingual string pair.  `get(lang)` returns the appropriate variant.
#[allow(dead_code)]
pub struct Msg {
    pub en: &'static str,
    pub th: &'static str,
}

impl Msg {
    #[allow(dead_code)]
    #[inline]
    pub fn get(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.en,
            Lang::Th => self.th,
        }
    }
}

/// Convenience free function: return `en` or `th` depending on `lang`.
#[inline]
pub fn t(lang: Lang, en: &'static str, th: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Th => th,
    }
}
