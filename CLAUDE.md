# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

A personal portfolio/resume single-page site for **Apiwat Kaiburt** (a job-seeking Software/Backend Engineer), built with **Dioxus 0.7 + WASM (web target)**.

The prototype has been **fully ported** (Nav, Hero + typed terminal, Experience timeline w/ expandable entry, Projects, Skills, Contact, Footer — all bilingual). The page is a single-page scroll site; runtime-verified rendering + behaviors. Remaining polish: scroll-reveal fade-in (intentionally deferred), EN copy review, a real Ferris image, real contact email.

### Current architecture
- `src/main.rs` — `App` provides the language signal and lays out the sections (no `Router`; hash-anchor nav).
- `src/i18n.rs` — `Lang` (Th/En) in a `Signal<Lang>` via context (`provide_lang`/`use_lang`); `Tx { th, en }` (build with `tx(..)`) for every display string, resolved with `.t(lang)`.
- `src/data.rs` — static bilingual content tables (`TIMELINE`, `PROJECTS`, `SKILLS`).
- `src/components/` — one file per section (`nav`, `hero`, `experience`, `projects`, `skills`, `contact`) + `widgets.rs` (`RustBadge`, `IconSlot`).
- `assets/styling/main.css` — hand-authored CSS ported verbatim from the prototype (NOT Tailwind utilities; Tailwind is intentionally not linked — its preflight reset fights the prototype's reset).

## Source-of-truth documents

- **`src/data.rs`** — the live bilingual content tables (`TIMELINE`, `PROJECTS`, `SKILLS`). The source of truth for what's displayed.
- **`AGENTS.md`** — Dioxus 0.7 API cheatsheet. 0.7 changed every API: **`cx`, `Scope`, and `use_state` are gone.** Use `use_signal`, `use_memo`, `use_resource`, `use_context`/`use_context_provider`.

> The original port docs were **deleted** by the user once the port was done and verified (2026-06-21): `DATA.md` (raw resume content), `CONTEXT.md` (port guide), and `reference/prototype.html` (the original HTML prototype). Their essentials live on here: displayed content in `src/data.rs`, design tokens + architecture + gotchas in this file, and the not-displayed resume backlog (cut projects, deeper detail, certificates) in the agent memory file `resume-content-backlog.md`.

## Commands

Build tooling is the Dioxus CLI (`dx`, currently 0.7.x), not plain `cargo run`.

```bash
dx serve                      # dev server, web (default platform), hot reload
dx serve --platform desktop   # run as native desktop app
dx build                      # production build
cargo check                   # fast type-check without dx
cargo clippy                  # lint (see clippy.toml constraint below)
cargo fmt                     # format
```

There are currently **no tests**.

Platform features are wired in `Cargo.toml`: `default = ["web"]`, plus optional `desktop` and `mobile`. Dependencies: `dioxus` (with the `router` feature — kept for future use, currently unused) and `gloo-timers` (web timers for the terminal animation).

## Dioxus 0.7 specifics that bite

- **Do NOT create an `index.html` at the crate root.** `dx` treats a root `index.html` as the page's index template and serves it *instead of* mounting the app (no `#main`), so the Dioxus app silently stops rendering. (This actually bit us with the original prototype file.) Without a root `index.html`, dx uses its built-in template (correct).
- **`clippy.toml` forbids holding signal borrows across `.await`.** Reads/writes from `generational_box` / `dioxus_signals` (`GenerationalRef`, `GenerationalRefMut`, `WriteLock`) held over an await point will deadlock the signal. Clone the value out of the signal before awaiting; don't keep a `.read()`/`.write()` guard alive across `.await`. (See the terminal typer in `hero.rs` — the write guard is dropped in a block before each timer `.await`.)
- **`use_resource`, not `use_future`, for animations that must restart on a signal change.** `use_future` runs once; `use_resource` re-runs reactively when a signal it reads changes *and cancels the previous run*. The terminal typer uses `use_resource` keyed on `lang()` so it replays cleanly on language toggle.
- **Automatic Tailwind**: Dioxus 0.7 compiles Tailwind itself by detecting `tailwind.css` next to `Cargo.toml` (its content is just `@import "tailwindcss";`). No manual Tailwind CLI / npm step is needed — just `dx serve`. The compiled output lives at `assets/tailwind.css`.
- **Assets** go through the `asset!("/assets/...")` macro (paths are crate-root-relative, leading `/`). The prototype inlines the Ferris mascot as base64; when porting, switch to `asset!("/assets/ferris-rust.png")` instead of carrying base64 in source.

## Component architecture

The page is a **scroll-based single page**, not a multi-route app. Nav links scroll to sections (`scroll_into_view`), they are not real routes — the template's `/blog/:id` route is boilerplate and not part of the final design. Sections, each its own component:

- `Nav` — sticky, backdrop-blur, self-logo + pulsing dot, 4 scroll links.
- `Hero` — eyebrow/heading/sub + two scroll buttons + `Terminal` sub-component.
- `ExperienceTimeline` — the most complex part: a single spine mixing `TimelineGroupLabel` (year dividers), `TimelineEntry` (color variants: olive normal / `alt` rust / `flagship` glow), and one `ExpandableEntry` (the co-op entry, `is_expanded: Signal<bool>`, with a nested `SubTimeline`).
- `ProjectsGrid` → `ProjectCard` — driven by `Vec<Project>`.
- `SkillsGrid` → `SkillGroup` → `SkillPill`, with an `IconSlot` component taking `Option<Asset>` (falls back to a dashed placeholder when no icon yet — most slots are still empty placeholders).
- `ContactSection` — dark closing card, mailto + GitHub link.
- `ScrollReveal` — cross-cutting fade-in-on-scroll; implement once as a hook (`use_intersection_observer` via web-sys) and wrap top-level components.

### Implementation gotchas
1. Establish design tokens first (as Rust consts or CSS variables — see palette below).
2. Port static sections (Hero text, Skills, Contact) to validate the `dx serve` pipeline.
3. Then `ExperienceTimeline` (toggle state).
4. Then the **Terminal typing animation** — must be **redesigned Dioxus-native** (`use_future`/`gloo_timers` + a `Signal<String>` appended char-by-char), *not* a copy of the prototype's `setInterval`/DOM-string JS into `eval`.
5. Add `ScrollReveal` last.

The prototype's expandable co-op toggle uses a `scrollHeight` JS hack for its `max-height` transition; in Dioxus prefer conditional render + CSS transition on `opacity`/`grid-template-rows` and skip the hack entirely.

## Design tokens

Palette (cream/ink/olive/rust theme):

```
cream #F2ECDF · cream-2 #EAE2D2 · ink #332B1C · ink-soft #6B5F47
label #8C7C5C · olive #4D5A2C · olive-deep #3A4420 · rust #C16A35
rust-deep #9A4F23 · line #D8CCAE · card #FBF8F0
```

Fonts: **JetBrains Mono** (all headings/mono) + **Nunito** (body). Loaded from Google Fonts in the prototype.
