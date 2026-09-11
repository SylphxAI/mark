//! Mark domain — the single grammar: form × art × paint × geometry × text × motion.
//!
//! Everything here is pure and deterministic (ADR-0003): no clock, no network,
//! no process env. The kernel lives with its capability.

pub mod catalog;
pub mod color;
pub mod icons;
pub mod motion;
pub mod pill;
pub mod recovery;
pub mod shapes;
pub mod spec;
pub mod svg;
pub mod theme;

pub(crate) use catalog::{
    normalize_animation, normalize_layout, MAX_DESC_CHARS, MAX_ICONS, MAX_LABEL_CHARS, MAX_LINES,
    MAX_MESSAGE_CHARS, MAX_SERVICE_CHARS, MAX_TEXT_CHARS,
};
pub(crate) use pill::{named_color, PillStyle};
pub(crate) use recovery::split_badge_path;
pub use svg::cap_text;
pub(crate) use svg::normalize_hex_token;

// Kernel surface consumed by the contract tests (see tests/*.rs). Recovery and
// studio types are reached through `domain::recovery` directly.
pub use spec::{DeploySpec, HeroSpec, MarkForm, MarkSpec, PillSpec, StripSpec};
