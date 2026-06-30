use dioxus::prelude::*;

use super::widgets::IconSlot;
use crate::data::SKILLS;
use crate::i18n::{tx, use_lang, Tx};

const NUM: Tx = tx("03 / สกิล", "03 / Skills");
const TITLE: Tx = tx("เครื่องมือที่ใช้", "Tools I use");

#[component]
pub fn Skills() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { id: "skills",
            div { class: "section-head",
                span { class: "section-num", "{NUM.t(l)}" }
                h2 { class: "section-title", "{TITLE.t(l)}" }
                div { class: "section-rule" }
            }
            div { class: "skills-grid",
                for g in SKILLS.iter() {
                    div {
                        div { class: "skill-group-label", "{g.label.t(l)}" }
                        div { class: "skill-pills",
                            for sk in g.skills.iter() {
                                span { class: "pill",
                                    IconSlot { icon: sk.icon }
                                    "{sk.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
