//! Pill vocabulary — style, geometry, and text attributes for pill-shaped marks.
//!
//! Named colors are paint tokens and live in `color`; this module owns the
//! shields-style geometry shared by `pill` and `deploy`.

use crate::capabilities::mark::domain::text::{
    line_advance, Metric, FONT_MONO, FONT_SHIELDS_SANS, FONT_UI_SANS_COMPACT,
};

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

/// Geometry every pill-shaped mark paints with.
///
/// `pill` and `deploy` are the same shields-style box: one authority for the
/// box height, corner radius, text baseline, and text attributes keeps them
/// from drifting apart.
pub(crate) struct PillMetrics {
    pub height: u32,
    pub radius: f32,
    pub baseline: u32,
    pub text_attrs: String,
}

impl PillMetrics {
    pub(crate) fn new(style: PillStyle, family: &str) -> Self {
        let height = match style {
            PillStyle::ForTheBadge => 28,
            _ => 20,
        };
        let radius = match style {
            PillStyle::Pill | PillStyle::Social => height as f32 / 2.0,
            PillStyle::ForTheBadge => 4.0,
            _ => 3.0,
        };
        let baseline = if style == PillStyle::ForTheBadge {
            18
        } else {
            14
        };
        let text_attrs = if style == PillStyle::ForTheBadge {
            format!(
                "font-family=\"{family}\" font-size=\"11\" font-weight=\"700\" letter-spacing=\"0.5\""
            )
        } else {
            format!("font-family=\"{family}\" font-size=\"11\" font-weight=\"500\"")
        };
        Self {
            height,
            radius,
            baseline,
            text_attrs,
        }
    }
}

/// `font-family` for a pill-shaped mark: `font=mono` wins, the badge weight uses
/// the compact stack, everything else uses the shields default.
pub(crate) fn family(font: Option<&str>, style: PillStyle) -> &'static str {
    match font.map(|f| f.to_ascii_lowercase()).as_deref() {
        Some("mono") => FONT_MONO,
        _ if style == PillStyle::ForTheBadge => FONT_UI_SANS_COMPACT,
        _ => FONT_SHIELDS_SANS,
    }
}
