use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaChevronRight;
use dioxus_free_icons::Icon;

use super::widgets::RustBadge;
use crate::data::{ExpEntry, TimelineNode, TIMELINE};
use crate::i18n::{tx, use_lang, Tx};

const NUM: Tx = tx("01 / ประสบการณ์", "01 / Experience");
const TITLE: Tx = tx("เส้นทางที่ผ่านมา", "The road so far");
const LEAD: Tx = tx(
    "▸ นำทีมพัฒนาในงานฟรีแลนซ์ทุกโปรเจกต์",
    "▸ Led the development team on every freelance project",
);
const HINT: Tx = tx("▸ กดเพื่อดูรายละเอียดทั้ง 4 โปรเจกต์", "▸ Tap to see all 4 projects");
const AUTHORS_LABEL: Tx = tx("ผู้ร่วมวิจัย", "Authors");
const CERT_LABEL: Tx = tx("ดูใบรับรอง ↗", "View certificate ↗");

const CERT_IEEE: Asset = asset!("/assets/cert-ieee.jpg");
const CERT_GLOBAL_HOUSE: Asset = asset!("/assets/cert-global-house.jpg");
const CERT_INTERNSHIP: Asset = asset!("/assets/cert-internship.jpg");

/// Resolve a certificate key (from the entry data) to its bundled image.
fn cert_asset(key: &str) -> Option<Asset> {
    match key {
        "ieee" => Some(CERT_IEEE),
        "global_house" => Some(CERT_GLOBAL_HOUSE),
        "internship" => Some(CERT_INTERNSHIP),
        _ => None,
    }
}

#[component]
pub fn Experience() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { id: "experience",
            div { class: "section-head",
                span { class: "section-num", "{NUM.t(l)}" }
                h2 { class: "section-title", "{TITLE.t(l)}" }
                div { class: "section-lead", "{LEAD.t(l)}" }
                div { class: "section-rule" }
            }
            div { class: "timeline",
                for node in TIMELINE.iter() {
                    {
                        match node {
                            TimelineNode::Group(label) => rsx! {
                                div { class: "t-group-label", "{label.t(l)}" }
                            },
                            TimelineNode::Entry(e) if e.sub.is_empty() => rsx! {
                                TimelineEntry { entry: *e }
                            },
                            TimelineNode::Entry(e) => rsx! {
                                ExpandableEntry { entry: *e }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineEntry(entry: ExpEntry) -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        div { class: entry.variant.class(),
            div { class: "exp-date", "{entry.date.t(l)}" }
            div { class: "exp-role",
                if entry.rust_badge {
                    RustBadge {}
                }
                "{entry.role.t(l)}"
            }
            div { class: "exp-org", "{entry.org.t(l)}" }
            p { class: "exp-desc", "{entry.desc.t(l)}" }
            if !entry.authors.is_empty() {
                div { class: "exp-authors", "{AUTHORS_LABEL.t(l)}: {entry.authors}" }
            }
            Highlights { items: entry.highlights }
            Tags { tags: entry.tags }
            Certs { certs: entry.certs }
        }
    }
}

#[component]
fn Highlights(items: &'static [crate::i18n::Tx]) -> Element {
    let lang = use_lang();
    let l = lang();
    if items.is_empty() {
        return rsx! {};
    }
    rsx! {
        ul { class: "exp-highlights",
            for item in items.iter() {
                li { "{item.t(l)}" }
            }
        }
    }
}

#[component]
fn Certs(certs: &'static [&'static str]) -> Element {
    let lang = use_lang();
    let l = lang();
    if certs.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "exp-certs",
            for key in certs.iter() {
                if let Some(asset) = cert_asset(key) {
                    a { class: "exp-cert", href: asset, target: "_blank",
                        img { src: asset, alt: "certificate" }
                        span { "{CERT_LABEL.t(l)}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ExpandableEntry(entry: ExpEntry) -> Element {
    let lang = use_lang();
    let l = lang();
    let mut expanded = use_signal(|| false);

    let item_class = if expanded() {
        format!("{} expanded", entry.variant.class())
    } else {
        entry.variant.class().to_string()
    };

    rsx! {
        div { class: "{item_class}",
            button {
                class: "exp-toggle",
                onclick: move |_| expanded.toggle(),
                span { class: "exp-toggle-chev",
                    Icon { width: 11, height: 11, icon: FaChevronRight }
                }
                span { class: "exp-toggle-main",
                    div { class: "exp-date", "{entry.date.t(l)}" }
                    div { class: "exp-role", "{entry.role.t(l)}" }
                    div { class: "exp-org", "{entry.org.t(l)}" }
                }
            }
            p { class: "exp-desc", "{entry.desc.t(l)}" }
            Highlights { items: entry.highlights }
            div { class: "exp-hint", "{HINT.t(l)}" }
            div { class: "sub-timeline",
                for s in entry.sub.iter() {
                    div { class: "sub-item",
                        div { class: "sub-role", "{s.role.t(l)}" }
                        p { class: "sub-desc", "{s.desc.t(l)}" }
                        Tags { tags: s.tags }
                    }
                }
            }
            Certs { certs: entry.certs }
        }
    }
}

#[component]
fn Tags(tags: &'static [&'static str]) -> Element {
    rsx! {
        div { class: "exp-tags",
            for t in tags.iter() {
                span { class: "tag", "{t}" }
            }
        }
    }
}
