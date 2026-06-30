//! Bilingual (TH/EN) plumbing.
//!
//! The current language lives in a `Signal<Lang>` provided at the app root via
//! [`use_context_provider`]. Any component that calls [`use_lang`] and reads the
//! signal in its render body auto-subscribes and re-renders when the language is
//! toggled.
//!
//! Display strings are modelled as [`Tx`] — a `{ th, en }` pair of `&'static str`
//! literals. TH copy is verbatim from the original prototype (the source of truth
//! for what is shown); EN copy is a first-draft translation pending review.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Th,
    En,
}

impl Lang {
    pub fn toggle(self) -> Self {
        match self {
            Lang::Th => Lang::En,
            Lang::En => Lang::Th,
        }
    }
}

/// A bilingual string literal pair. Build with [`tx`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tx {
    pub th: &'static str,
    pub en: &'static str,
}

impl Tx {
    /// Resolve to the string for the given language.
    pub fn t(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Th => self.th,
            Lang::En => self.en,
        }
    }
}

/// Terse constructor so data tables read as `tx("ไทย", "english")`.
pub const fn tx(th: &'static str, en: &'static str) -> Tx {
    Tx { th, en }
}

/// Install the language signal at the app root. Call once in `App`.
pub fn provide_lang() -> Signal<Lang> {
    use_context_provider(|| Signal::new(Lang::Th))
}

/// Read the shared language signal from any descendant component.
pub fn use_lang() -> Signal<Lang> {
    use_context::<Signal<Lang>>()
}
