use dioxus::prelude::*;

use crate::i18n::{tx, use_lang, Lang, Tx};

/// Section links. `href` uses native hash navigation — combined with
/// `html{scroll-behavior:smooth}` this gives smooth scrolling with no JS.
const LINKS: &[(&str, &str, Tx)] = &[
    ("01", "#experience", tx("ประสบการณ์", "Experience")),
    ("02", "#projects", tx("โปรเจกต์", "Projects")),
    ("03", "#skills", tx("สกิล", "Skills")),
    ("04", "#contact", tx("ติดต่อ", "Contact")),
];

#[component]
pub fn Nav() -> Element {
    let mut lang = use_lang();
    let l = lang();

    rsx! {
        nav {
            div { class: "nav-inner",
                div { class: "nav-mark",
                    span { class: "dot" }
                    "apiwat"
                    span { style: "color:var(--label)", ".kaiburt" }
                }
                div { class: "nav-right",
                    div { class: "nav-links",
                        for (num , href , label) in LINKS.iter() {
                            a { href: "{href}",
                                span { class: "num", "{num}" }
                                "{label.t(l)}"
                            }
                        }
                    }
                    button {
                        class: "lang-toggle",
                        onclick: move |_| lang.set(lang().toggle()),
                        span { class: if l == Lang::Th { "on" }, "TH" }
                        span { class: "sep", "/" }
                        span { class: if l == Lang::En { "on" }, "EN" }
                    }
                }
            }
        }
    }
}
