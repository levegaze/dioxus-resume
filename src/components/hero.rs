use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::i18n::{tx, use_lang, Tx};

// Title is split so the accent word keeps its olive colour and the line break.
const TITLE_PRE: Tx = tx("สร้างระบบที่ ", "Systems that ");
const TITLE_ACCENT: Tx = tx("คุ้มทุก resource", "make every resource count");
const TITLE_POST: Tx = tx("แม้เซิร์ฟเวอร์จะเล็กแค่ไหน", "no matter how small the server");

const EYEBROW: Tx = tx(
    "backend developer · software engineer · เปิดรับงาน",
    "backend developer · software engineer · open to work",
);

// Sub-headline split around the two bolded technologies.
const SUB_A: Tx = tx("Software engineer สาย ", "A software engineer working in ");
const SUB_B: Tx = tx(" และ ", " and ");
const SUB_C: Tx = tx(
    " — ถนัดสร้าง API service ที่ performance สูงและรีดทุก resource ให้คุ้มค่าแม้บนเครื่องที่จำกัด ส่งมอบระบบใช้งานจริงและลงพื้นที่คุยงานเก็บ requirement กับลูกค้าเองทุกโปรเจกต์",
    " — focused on building high-performance API services that make the most of every resource, even on constrained machines; shipping production systems and gathering requirements with clients on-site on every project.",
);

const BTN_PROJECTS: Tx = tx("ดูโปรเจกต์ →", "View projects →");
const BTN_CONTACT: Tx = tx("ติดต่อฉัน", "Get in touch");

#[component]
pub fn Hero() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { class: "hero",
            div { class: "eyebrow", "{EYEBROW.t(l)}" }
            h1 { class: "title",
                "{TITLE_PRE.t(l)}"
                span { class: "accent", "{TITLE_ACCENT.t(l)}" }
                br {}
                "{TITLE_POST.t(l)}"
            }
            p { class: "hero-sub",
                "{SUB_A.t(l)}"
                b { "Rust" }
                "{SUB_B.t(l)}"
                b { "Node.js" }
                "{SUB_C.t(l)}"
            }
            div { class: "hero-actions",
                a { class: "btn btn-primary", href: "#projects", "{BTN_PROJECTS.t(l)}" }
                a { class: "btn btn-ghost", href: "#contact", "{BTN_CONTACT.t(l)}" }
            }

            Terminal {}
        }
    }
}

#[derive(Clone, Copy)]
enum Kind {
    Cmd,
    Out,
    Ok,
}

const TERM_LINES: &[(Kind, Tx)] = &[
    (Kind::Cmd, tx("whoami", "whoami")),
    (Kind::Out, tx("apiwat kaiburt — software engineer", "apiwat kaiburt — software engineer")),
    (Kind::Cmd, tx("cat stack.txt", "cat stack.txt")),
    (Kind::Out, tx("rust · typescript · postgresql · redis", "rust · typescript · postgresql · redis")),
    (Kind::Cmd, tx("echo $STATUS", "echo $STATUS")),
    (Kind::Ok, tx("เปิดรับโอกาสงาน = true", "open_to_work = true")),
];

#[derive(Clone, PartialEq)]
enum Row {
    Cmd(String),
    Out(String),
    Ok(String),
}

/// The "whoami.sh" signature panel — types out line by line. Driven by
/// `use_resource` (not `use_future`): it reactively re-runs when `lang()`
/// changes, cancelling the in-flight typer so the animation cleanly replays in
/// the new language instead of interleaving.
#[component]
fn Terminal() -> Element {
    let lang = use_lang();
    let mut rows = use_signal(Vec::<Row>::new);
    let mut done = use_signal(|| false);

    let _typer = use_resource(move || async move {
        let l = lang(); // dependency → re-runs (replays) on language toggle
        rows.set(Vec::new());
        done.set(false);
        TimeoutFuture::new(400).await;

        for (kind, txt) in TERM_LINES.iter() {
            let text = txt.t(l);
            match kind {
                Kind::Cmd => {
                    rows.write().push(Row::Cmd(String::new()));
                    // chars(), not byte slicing — Thai is multi-byte.
                    for ch in text.chars() {
                        {
                            let mut w = rows.write();
                            if let Some(Row::Cmd(s)) = w.last_mut() {
                                s.push(ch);
                            }
                        } // drop write guard before await (clippy await-holding rule)
                        TimeoutFuture::new(32).await;
                    }
                    TimeoutFuture::new(220).await;
                }
                Kind::Out => {
                    rows.write().push(Row::Out(text.to_string()));
                    TimeoutFuture::new(320).await;
                }
                Kind::Ok => {
                    rows.write().push(Row::Ok(text.to_string()));
                    TimeoutFuture::new(320).await;
                }
            }
        }
        done.set(true);
    });

    rsx! {
        div { class: "term",
            div { class: "term-bar",
                span { class: "term-dot", style: "background:#E5635A" }
                span { class: "term-dot", style: "background:#E8B84B" }
                span { class: "term-dot", style: "background:#8FA05A" }
                span { class: "term-title", "whoami.sh" }
            }
            div { class: "term-body",
                for row in rows.read().iter() {
                    {
                        match row {
                            Row::Cmd(s) => rsx! {
                                div { class: "term-line",
                                    span { class: "term-prompt", "❯" }
                                    span { class: "term-cmd", "{s}" }
                                }
                            },
                            Row::Out(s) => rsx! {
                                div { class: "term-line",
                                    span { class: "term-out", "{s}" }
                                }
                            },
                            Row::Ok(s) => rsx! {
                                div { class: "term-line",
                                    span { class: "term-out term-ok", "{s}" }
                                }
                            },
                        }
                    }
                }
                if done() {
                    div { style: "display:flex;gap:10px",
                        span { class: "term-prompt", "❯" }
                        span { class: "term-cursor" }
                    }
                }
            }
        }
    }
}
