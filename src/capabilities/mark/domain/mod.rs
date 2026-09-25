//! Mark domain — the single grammar: form × art × paint × geometry × text × motion.
//!
//! Everything here is pure and deterministic (ADR-0003): no clock, no network,
//! no process env. The kernel lives with its capability.

pub mod art;
pub mod brand_icons;
pub mod catalog;
pub mod color;
pub mod hash;
pub mod icon_aliases;
pub mod icons;
pub mod motion;
pub mod paint;
pub mod pill;
pub mod recovery;
pub mod shapes;
pub mod shields;
pub mod spec;
pub mod svg;
pub mod text;
pub mod theme;
pub mod typing;
pub mod widths;

pub(crate) use catalog::{
    normalize_layout, MAX_DESC_CHARS, MAX_ICONS, MAX_LABEL_CHARS, MAX_LINES, MAX_MESSAGE_CHARS,
    MAX_SERVICE_CHARS, MAX_TEXT_CHARS,
};
pub(crate) use motion::normalize_animation;
pub(crate) use pill::PillStyle;
pub(crate) use shields::split_badge_path;
pub(crate) use spec::{HeroOverrides, PlacedText};
pub(crate) use svg::normalize_hex_token;

// Kernel surface consumed by the contract tests (see tests/*.rs). Recovery and
// studio types are reached through `domain::recovery` directly.
pub use spec::{
    DeploySpec, HeroSpec, MarkForm, MarkSpec, PillSpec, ScoreSpec, StripSpec, TypingSpec,
};
pub use text::cap_text;
