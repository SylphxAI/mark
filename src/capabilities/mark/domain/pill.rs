//! Pill vocabulary — the atomic mark's style and named colors.

use crate::capabilities::mark::domain::text::{line_advance, Metric};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum PillStyle {
    #[default]
    Flat,
    Plastic,
    ForTheBadge,
    Social,
    Pill,
}

impl PillStyle {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "plastic" => Self::Plastic,
            "for-the-badge" | "forthebadge" => Self::ForTheBadge,
            "social" => Self::Social,
            "pill" => Self::Pill,
            _ => Self::Flat,
        }
    }
}

/// Pill text width in px.
///
/// This is the one geometry authority for pill-shaped marks (`pill` and
/// `deploy`): a flat per-character estimate cannot track glyph advances, so
/// both forms measure with the shared `Display` metric.
pub(crate) fn measure(text: &str, style: PillStyle) -> u32 {
    let font_size = 11.0;
    let tracking = if style == PillStyle::ForTheBadge {
        0.5
    } else {
        0.0
    };
    let pad = if style == PillStyle::ForTheBadge {
        20.0
    } else {
        14.0
    };
    let n = text.chars().count() as f32;
    let extra = if n > 1.0 { tracking * (n - 1.0) } else { 0.0 };
    (line_advance(text, font_size, Metric::Display) + extra + pad)
        .ceil()
        .max(1.0) as u32
}
