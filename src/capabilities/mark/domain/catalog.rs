//! The Mark vocabulary — one catalog, one contract.

use crate::capabilities::mark::domain::icons;
use crate::capabilities::mark::domain::motion::ANIMATIONS;
use crate::capabilities::mark::domain::shapes::{ART_TYPES, FEATURED_ART_TYPES};
use crate::capabilities::mark::domain::spec::MarkForm;

pub use crate::capabilities::mark::domain::motion::normalize_animation;
pub use crate::capabilities::mark::domain::shapes::{is_art_type, normalize_art_type};

/// Layout families (hero composition, not background recipe).
pub const LAYOUTS: &[&str] = &["default", "plate", "signal", "terminal"];

/// Pill styles (shields vocabulary).
pub const BADGE_STYLES: &[&str] = &["flat", "plastic", "for-the-badge", "social", "pill"];

/// Content typography (ADR-0004): neutral, no embedded fonts.
pub const FONTS: &[&str] = &["sans", "mono"];

/// Bounded input contract (ADR-0002/0003): truncation is marked with `…` and
/// total length never exceeds the cap. These are the public limits.
pub const MAX_TEXT_CHARS: usize = 500;
pub const MAX_DESC_CHARS: usize = 240;
pub const MAX_LINES: usize = 8;
pub const MAX_LABEL_CHARS: usize = 80;
pub const MAX_MESSAGE_CHARS: usize = 120;
pub const MAX_ICONS: usize = 60;
pub const MAX_SERVICE_CHARS: usize = 40;

pub fn normalize_layout(raw: Option<&str>) -> &'static str {
    match raw
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .as_deref()
    {
        None | Some("default") | Some("center") => "default",
        Some("plate") | Some("product") | Some("card") | Some("oss") => "plate",
        Some("signal") | Some("hero") => "signal",
        Some("terminal") | Some("cli") | Some("mono") => "terminal",
        _ => "default",
    }
}

/// The full vocabulary as one machine-readable contract (studio + catalog).
pub fn vocabulary() -> serde_json::Value {
    serde_json::json!({
        "forms": MarkForm::ALL,
        "art_types": ART_TYPES,
        "featured_art_types": FEATURED_ART_TYPES,
        "layouts": LAYOUTS,
        "themes": super::theme::list_names(),
        "icons": icons::available(),
        "badge_styles": BADGE_STYLES,
        "animations": ANIMATIONS,
        "fonts": FONTS,
    })
}
