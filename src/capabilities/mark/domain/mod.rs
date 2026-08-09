//! Mark domain — the single grammar: form × art × paint × geometry × text × motion.
//!
//! Everything here is pure and deterministic (ADR-0003): no clock, no network,
//! no process env. The kernel lives with its capability.

pub mod catalog;
pub mod color;
pub mod icons;
pub mod motion;
pub mod pill;
pub mod shapes;
pub mod spec;
pub mod svg;
pub mod theme;

pub use catalog::{
    is_art_type, normalize_animation, normalize_art_type, normalize_layout, BADGE_STYLES,
    BRANDS, LAYOUTS, MAX_BRAND_CHARS, MAX_DESC_CHARS, MAX_ICONS, MAX_LABEL_CHARS, MAX_LINES,
    MAX_MESSAGE_CHARS, MAX_SERVICE_CHARS, MAX_TAGLINE_CHARS, MAX_TEXT_CHARS,
};
pub use pill::{named_color, PillStyle};
pub use spec::{DeploySpec, HeroSpec, IdentitySpec, MarkForm, MarkSpec, PillSpec, StripSpec};
pub use svg::{cap_text, esc, normalize_hex_token};
