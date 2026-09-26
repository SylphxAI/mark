//! The Mark vocabulary — one catalog, one contract.

use crate::capabilities::mark::domain::art::art_ids;
use crate::capabilities::mark::domain::motion::ANIMATIONS;
use crate::capabilities::mark::domain::spec::MarkForm;
use crate::capabilities::mark::domain::{brand_icons, icons};

/// Layout families (hero composition, not background recipe).
pub(crate) const LAYOUTS: &[&str] = &["default", "left"];

/// Pill styles (shields vocabulary).
pub(crate) const BADGE_STYLES: &[&str] = &[
    "flat",
    "flat-square",
    "plastic",
    "for-the-badge",
    "social",
    "pill",
];

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

/// Normalize `layout=` against [`LAYOUTS`]: `default` centres the text,
/// `left` aligns it to the start of the column.
///
/// `plate` and `terminal` (published before the curated set) were left
/// aligned and map to `left`; anything else renders the default.
pub(crate) fn normalize_layout(raw: Option<&str>) -> &'static str {
    match raw
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .as_deref()
    {
        Some("left" | "plate" | "terminal") => "left",
        _ => "default",
    }
}

/// The full vocabulary as one machine-readable contract (studio + catalog).
pub(crate) fn vocabulary() -> serde_json::Value {
    serde_json::json!({
        "forms": MarkForm::ALL,
        "art_types": art_ids(),
        "layouts": LAYOUTS,
        "themes": super::theme::list_names(),
        "theme_palettes": super::theme::palettes(),
        "icons": icons::available(),
        "icon_count": icons::count(),
        "skill_icons": icons::skill_ids().collect::<Vec<_>>(),
        "icon_source": {
            "name": "Simple Icons",
            "version": brand_icons::version(),
            "license": "CC0-1.0",
            "icons_url": "https://simpleicons.org",
        },
        "badge_styles": BADGE_STYLES,
        "animations": ANIMATIONS,
        "fonts": FONTS,
    })
}
