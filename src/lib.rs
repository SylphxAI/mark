//! Sylphx Mark — embeddable image API (URL → SVG).
//!
//! One concept, one grammar (ADR-0003): form × art × paint × geometry × text ×
//! motion. Every mark is a pure function of its URL — deterministic, immutable,
//! never failing. No clock, no upstream, no state.

pub mod bootstrap;
pub mod capabilities;
pub mod interfaces;

// Single capability-rooted public surface for tests and internal callers.
pub mod mark {
    pub use crate::capabilities::mark::*;
}

// Kernel re-exports used by integration tests (single authority: mark domain).
pub mod color {
    pub use crate::capabilities::mark::domain::color::*;
}
pub mod themes {
    pub use crate::capabilities::mark::domain::theme::*;
}
pub mod svg {
    pub use crate::capabilities::mark::domain::svg::*;
}

pub use bootstrap::AppState;
pub use interfaces::http::app;
