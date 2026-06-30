use dioxus::prelude::*;

use crate::i18n::{tx, use_lang, Tx};

const TITLE: Tx = tx("มาสร้างอะไรที่อยู่ยืนกันเถอะ", "Let's build something that lasts");
const BODY: Tx = tx(
    "เปิดรับโอกาสงานด้าน software / backend engineering — พร้อมคุยเรื่อง actor system, event sourcing หรือทำไม C ยังสำคัญอยู่",
    "Open to software / backend engineering roles — happy to talk actor systems, event sourcing, or why C still matters.",
);
const BTN_MAIL: Tx = tx("ส่งอีเมลถึงฉัน", "Email me");

// TODO: real email — still the prototype placeholder.
const EMAIL: &str = "apiwat@example.com";

#[component]
pub fn Contact() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { id: "contact",
            div { class: "contact-box",
                h2 { "{TITLE.t(l)}" }
                p { "{BODY.t(l)}" }
                div { class: "contact-links",
                    a { class: "btn btn-primary", href: "mailto:{EMAIL}", "{BTN_MAIL.t(l)}" }
                    a {
                        class: "btn btn-ghost",
                        href: "https://github.com/levegaze",
                        target: "_blank",
                        "GitHub ↗"
                    }
                }
            }
        }
    }
}
