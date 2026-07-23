//! Sylphx Mark — embeddable image API (URL → SVG).
//!
//! Architecture: Capability-first Modular DDD with Clean/Hexagonal boundaries
//! and Functional Core / Imperative Shell (see `docs/adr/ADR-0001-capability-first-architecture.md`).

pub mod bootstrap;
pub mod capabilities;
pub mod interfaces;
pub mod shared;

// Capability-rooted public surface for tests and internal callers.
pub mod badge {
    pub use crate::capabilities::badge::{render, BadgeInput, BadgeStyle};
}
pub mod banner {
    pub use crate::capabilities::banner::{
        render, BannerInput, ANIMATIONS, BANNER_TYPES, FEATURED_TYPES, LAYOUTS,
    };
}
pub mod brand {
    pub use crate::capabilities::brand_kit::render as render_brand_card;
}
pub mod icons {
    pub use crate::capabilities::icon_row::{available, render_row};
}
pub mod deploy_mark {
    pub use crate::capabilities::deploy_mark::render;
}
pub mod github_card {
    pub use crate::capabilities::github_card::{
        org_stats, repo_card, user_stats, CardOpts, GitHubSource, HttpGitHubSource,
    };
}

// Shared kernel re-exports used by integration tests.
pub mod color {
    pub use crate::shared::color::*;
}
pub mod themes {
    pub use crate::shared::theme::*;
}
pub mod svg {
    pub use crate::shared::svg::*;
}

pub use bootstrap::AppState;
pub use interfaces::http::app;
