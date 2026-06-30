use dioxus::prelude::*;

const FERRIS: Asset = asset!("/assets/ferris-rust.png");
const LOGO_C: Asset = asset!("/assets/logo-c.png");
const LOGO_JS: Asset = asset!("/assets/logo-js.png");
const LOGO_TS: Asset = asset!("/assets/logo-ts.png");
const LOGO_MYSQL: Asset = asset!("/assets/logo-mysql.png");
const LOGO_REACT: Asset = asset!("/assets/logo-react.png");
const LOGO_SQL: Asset = asset!("/assets/logo-sql.png");
const LOGO_EXPRESS: Asset = asset!("/assets/logo-express.png");
const LOGO_SQLX: Asset = asset!("/assets/logo-sqlx.png");
const LOGO_PRISMA: Asset = asset!("/assets/logo-prisma.png");
const LOGO_POSTGRES: Asset = asset!("/assets/logo-postgres.png");
const LOGO_REDIS: Asset = asset!("/assets/logo-redis.png");
const LOGO_DOCKER: Asset = asset!("/assets/logo-docker.png");
const LOGO_NEXTJS: Asset = asset!("/assets/logo-nextjs.png");
const LOGO_DIOXUS: Asset = asset!("/assets/logo-dioxus.png");
const LOGO_AXUM: Asset = asset!("/assets/logo-axum.png");
const LOGO_TOKIO: Asset = asset!("/assets/logo-tokio.png");
const LOGO_DIESEL: Asset = asset!("/assets/logo-diesel.png");
const LOGO_TANSTACK: Asset = asset!("/assets/logo-tanstack.png");
const LOGO_CLAUDE: Asset = asset!("/assets/logo-claude.png");
const LOGO_LINUX: Asset = asset!("/assets/logo-linux.png");
const LOGO_CODEX: Asset = asset!("/assets/logo-codex.png");

/// Resolve a logo key (from the skill data) to its bundled asset.
fn logo(key: &str) -> Option<Asset> {
    Some(match key {
        "rust" => FERRIS,
        "c" => LOGO_C,
        "js" => LOGO_JS,
        "ts" => LOGO_TS,
        "mysql" => LOGO_MYSQL,
        "react" => LOGO_REACT,
        "sql" => LOGO_SQL,
        "express" => LOGO_EXPRESS,
        "sqlx" => LOGO_SQLX,
        "prisma" => LOGO_PRISMA,
        "postgres" => LOGO_POSTGRES,
        "redis" => LOGO_REDIS,
        "docker" => LOGO_DOCKER,
        "nextjs" => LOGO_NEXTJS,
        "dioxus" => LOGO_DIOXUS,
        "axum" => LOGO_AXUM,
        "tokio" => LOGO_TOKIO,
        "diesel" => LOGO_DIESEL,
        "tanstack" => LOGO_TANSTACK,
        "claude" => LOGO_CLAUDE,
        "linux" => LOGO_LINUX,
        "codex" => LOGO_CODEX,
        _ => return None,
    })
}

/// Rust marker (hard-hat Ferris) shown before Rust-flavoured roles/projects.
#[component]
pub fn RustBadge() -> Element {
    rsx! {
        span { class: "rust-badge",
            img { src: FERRIS, alt: "Rust" }
        }
    }
}

/// Icon slot for a skill pill. Shows the logo when `icon` resolves to an asset;
/// otherwise falls back to a dashed placeholder (icons filled in over time).
#[component]
pub fn IconSlot(icon: Option<&'static str>) -> Element {
    match icon.and_then(logo) {
        Some(asset) => rsx! {
            span { class: "icon-slot logo",
                img { src: asset, alt: icon.unwrap_or_default() }
            }
        },
        None => rsx! {
            span { class: "icon-slot" }
        },
    }
}
