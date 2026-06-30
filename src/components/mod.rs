//! Shared UI components for the portfolio. One file per page section, plus small
//! shared widgets.

mod widgets;

mod nav;
pub use nav::Nav;

mod hero;
pub use hero::Hero;

mod experience;
pub use experience::Experience;

mod projects;
pub use projects::Projects;

mod skills;
pub use skills::Skills;

mod contact;
pub use contact::Contact;
