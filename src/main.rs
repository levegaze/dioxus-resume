use dioxus::prelude::*;

use components::{Contact, Experience, Hero, Nav, Projects, Skills};
use i18n::{provide_lang, tx, use_lang, Tx};

mod components;
mod data;
mod i18n;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

const FOOTER: Tx = tx(
    "ทำด้วยความตั้งใจ · apiwat kaiburt © 2026",
    "built with care · apiwat kaiburt © 2026",
);

fn main() {
    dioxus::launch(App);
}

/// Root of the single-page portfolio. Provides the shared language signal and
/// lays out the page sections (no router — navigation is hash-anchor scrolling).
#[component]
fn App() -> Element {
    provide_lang();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "grain" }
        Nav {}
        div { class: "wrap",
            Hero {}
            Experience {}
            Projects {}
            Skills {}
            Contact {}
        }
        Footer {}
    }
}

#[component]
fn Footer() -> Element {
    let lang = use_lang();
    rsx! {
        footer { "{FOOTER.t(lang())}" }
    }
}
