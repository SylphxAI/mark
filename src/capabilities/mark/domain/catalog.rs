//! The Mark vocabulary — one catalog, one contract.

use crate::capabilities::mark::domain::icons;
use crate::capabilities::mark::domain::motion::ANIMATIONS;
use crate::capabilities::mark::domain::shapes::{ART_TYPES, FEATURED_ART_TYPES};
use crate::capabilities::mark::domain::spec::MarkForm;

/// Layout families (hero composition, not background recipe).
pub(crate) const LAYOUTS: &[&str] = &["default", "plate", "signal", "terminal"];

/// Pill styles (shields vocabulary).
pub(crate) const BADGE_STYLES: &[&str] = &["flat", "plastic", "for-the-badge", "social", "pill"];

/// Content typography (ADR-0004): neutral, no embedded fonts.
pub(crate) const FONTS: &[&str] = &["sans", "mono"];

/// Bounded input contract (ADR-0002/0003): truncation is marked with `…` and
/// total length never exceeds the cap. These are the public limits.
pub(crate) const MAX_TEXT_CHARS: usize = 500;
pub(crate) const MAX_DESC_CHARS: usize = 240;
pub(crate) const MAX_LINES: usize = 8;
pub(crate) const MAX_LABEL_CHARS: usize = 80;
pub(crate) const MAX_MESSAGE_CHARS: usize = 120;
pub(crate) const MAX_ICONS: usize = 60;
pub(crate) const MAX_SERVICE_CHARS: usize = 40;

/// Normalize `layout=` against [`LAYOUTS`].
///
/// The published list is the whole vocabulary; retired predecessor aliases
/// (`center`, `product`, `card`, `oss`, `hero`, `cli`, `mono`) are unknown
/// input and render the default layout.
pub(crate) fn normalize_layout(raw: Option<&str>) -> &'static str {
    match raw
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .as_deref()
    {
        Some("plate") => "plate",
        Some("signal") => "signal",
        Some("terminal") => "terminal",
        _ => "default",
    }
}

/// The full vocabulary as one machine-readable contract (studio + catalog).
pub(crate) fn vocabulary() -> serde_json::Value {
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
