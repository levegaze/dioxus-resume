use dioxus::prelude::*;

use super::widgets::RustBadge;
use crate::data::PROJECTS;
use crate::i18n::{tx, use_lang, Tx};

const NUM: Tx = tx("02 / โปรเจกต์", "02 / Projects");
const TITLE: Tx = tx("โปรเจกต์ส่วนตัว", "Personal Projects");

#[component]
pub fn Projects() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { id: "projects",
            div { class: "section-head",
                span { class: "section-num", "{NUM.t(l)}" }
                h2 { class: "section-title", "{TITLE.t(l)}" }
                div { class: "section-rule" }
            }
            div { class: "proj-grid",
                for p in PROJECTS.iter() {
                    div { class: "proj-card",
                        div { class: "proj-top",
                            span { class: "proj-name",
                                if p.rust_badge {
                                    RustBadge {}
                                }
                                "{p.name}"
                            }
                            if let Some(flag) = p.flag {
                                span { class: flag.class(), "{flag.label()}" }
                            }
                        }
                        p { class: "proj-desc", "{p.desc.t(l)}" }
                        if !p.highlights.is_empty() {
                            ul { class: "proj-highlights",
                                for item in p.highlights.iter() {
                                    li { "{item.t(l)}" }
                                }
                            }
                        }
                        div { class: "proj-stack",
                            for s in p.stack.iter() {
                                span { class: "pill", "{s}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
