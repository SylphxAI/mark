//! Pill vocabulary — style, geometry, and text paint for pill-shaped marks.
//!
//! This module is the one geometry authority for every pill-shaped mark
//! (`pill`, `score`, `deploy`). The geometry is shields' badge-maker, so a
//! shields URL moved to Mark keeps its exact box: the same heights, radii,
//! paddings, gradients, text shadow, and Verdana/Helvetica advance tables
//! ([`crate::capabilities::mark::domain::widths`]). Text is painted the
//! shields way — `font-size` ×10 inside `scale(.1)` with `textLength` — so the
//! glyph run always fills exactly the measured box, whatever font the viewer
//! has installed.
//!
//! Named colors are paint tokens and live in `paint`.

use crate::capabilities::mark::domain::paint::brightness;
use crate::capabilities::mark::domain::svg::esc;
use crate::capabilities::mark::domain::text::{FONT_MONO, FONT_SHIELDS_SANS, FONT_SHIELDS_SOCIAL};
use crate::capabilities::mark::domain::widths::Face;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum PillStyle {
    #[default]
    Flat,
    FlatSquare,
    Plastic,
    ForTheBadge,
    Social,
    /// Mark's own: flat geometry, fully rounded ends, no gloss.
    Pill,
}

impl PillStyle {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "flat-square" | "flatsquare" => Self::FlatSquare,
            "plastic" => Self::Plastic,
            "for-the-badge" | "forthebadge" => Self::ForTheBadge,
            "social" => Self::Social,
            "pill" => Self::Pill,
            _ => Self::Flat,
        }
    }

    /// Box and paint constants for this style (badge-maker values).
    pub(crate) fn chrome(self) -> Chrome {
        let (height, radius) = match self {
            Self::Flat => (20, 3.0),
            Self::FlatSquare => (20, 0.0),
            Self::Plastic => (18, 4.0),
            Self::ForTheBadge => (28, 0.0),
            Self::Social => (20, 2.0),
            Self::Pill => (20, 10.0),
        };
        let (defs, overlay) = match self {
            Self::Flat => (FLAT_DEFS, true),
            Self::Plastic => (PLASTIC_DEFS, true),
            _ => ("", false),
        };
        Chrome {
            height,
            radius,
            text_y: match self {
                Self::Plastic => 130,
                Self::ForTheBadge => 175,
                _ => 140,
            },
            shadow: matches!(self, Self::Flat | Self::Plastic),
            defs,
            overlay,
        }
    }

    /// Diameter of a message-side progress ring (the score badge).
    pub(crate) fn ring_diameter(self) -> f64 {
        match self {
            Self::ForTheBadge => 14.0,
            Self::Plastic => 11.0,
            _ => 12.0,
        }
    }
}

/// Which side of a badge a run of text sits on (for-the-badge measures the
/// message bold).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Part {
    Label,
    Message,
}

/// Box and paint constants for one style.
pub(crate) struct Chrome {
    pub height: u32,
    pub radius: f64,
    /// Text baseline in the ×10 text space.
    pub text_y: u32,
    /// Flat and plastic paint shields' soft text shadow.
    pub shadow: bool,
    /// `<defs>`-level paint (blur filter + gloss gradient), if any.
    pub defs: &'static str,
    /// Whether the gloss gradient `#s` is laid over the background.
    pub overlay: bool,
}

const FLAT_DEFS: &str = "<filter id=\"blur\"><feGaussianBlur stdDeviation=\"16\"/></filter>\
<linearGradient id=\"s\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/>\
<stop offset=\"1\" stop-opacity=\".1\"/></linearGradient>";

const PLASTIC_DEFS: &str = "<filter id=\"blur\"><feGaussianBlur stdDeviation=\"16\"/></filter>\
<linearGradient id=\"s\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#fff\" stop-opacity=\".7\"/>\
<stop offset=\".1\" stop-color=\"#aaa\" stop-opacity=\".1\"/><stop offset=\".9\" stop-color=\"#000\" stop-opacity=\".3\"/>\
<stop offset=\"1\" stop-color=\"#000\" stop-opacity=\".5\"/></linearGradient>";

fn face(style: PillStyle, part: Part, mono: bool) -> Face {
    match (style, part, mono) {
        (PillStyle::ForTheBadge, _, true) => Face::Mono(10),
        (_, _, true) => Face::Mono(11),
        (PillStyle::ForTheBadge, Part::Label, _) => Face::Verdana10,
        (PillStyle::ForTheBadge, Part::Message, _) => Face::Verdana10Bold,
        (PillStyle::Social, _, _) => Face::Helvetica11Bold,
        _ => Face::Verdana11,
    }
}

/// Painted width of one badge text run, in px — the one pill width authority.
///
/// badge-maker rules: most styles floor the advance and round up to an odd
/// pixel (pixel-grid alignment); for-the-badge floors and adds 1.25 px of
/// letter spacing per character (the text is already upper-cased). Like
/// badge-maker, an empty run still rounds up to 1 px outside for-the-badge.
pub(crate) fn measure(text: &str, style: PillStyle, part: Part, mono: bool) -> f64 {
    let w = face(style, part, mono).width(text).floor();
    if style == PillStyle::ForTheBadge {
        // badge-maker skips measuring an empty for-the-badge part.
        w + 1.25 * text.chars().count() as f64
    } else if w % 2.0 == 0.0 {
        w + 1.0
    } else {
        w
    }
}

/// The foreground `<g>` that every text run of a badge sits in.
pub(crate) fn text_group_open(style: PillStyle, mono: bool) -> String {
    let family = match (mono, style) {
        (true, _) => FONT_MONO,
        (false, PillStyle::Social) => FONT_SHIELDS_SOCIAL,
        (false, _) => FONT_SHIELDS_SANS,
    };
    match style {
        PillStyle::Social => format!(
            "<g fill=\"#333\" text-anchor=\"middle\" font-family=\"{family}\" text-rendering=\"geometricPrecision\" font-weight=\"700\" font-size=\"110px\">"
        ),
        PillStyle::ForTheBadge => format!(
            "<g fill=\"#fff\" text-anchor=\"middle\" font-family=\"{family}\" text-rendering=\"geometricPrecision\" font-size=\"100\">"
        ),
        _ => format!(
            "<g fill=\"#fff\" text-anchor=\"middle\" font-family=\"{family}\" text-rendering=\"geometricPrecision\" font-size=\"110\">"
        ),
    }
}

/// Text and shadow ink for a background (badge-maker `colorsForBackground`).
pub(crate) fn ink(bg: &str) -> (&'static str, &'static str) {
    if brightness(bg) <= 0.69 {
        ("#fff", "#010101")
    } else {
        ("#333", "#ccc")
    }
}

/// One painted text run, centred at `cx` px with `width` px of glyph run.
///
/// `bold` is the for-the-badge message weight. Shadowed styles paint the
/// shields blur + offset shadow under the text.
pub(crate) fn text_run(style: PillStyle, cx: f64, width: f64, text: &str, bg: &str) -> String {
    let chrome = style.chrome();
    let (x, y, len) = (fmt(cx * 10.0), chrome.text_y, fmt(width * 10.0));
    let (fg, shadow) = ink(bg);
    let content = esc(text);
    let fill = if fg == "#fff" {
        String::new()
    } else {
        format!(" fill=\"{fg}\"")
    };
    if chrome.shadow {
        let sy = y + 10;
        format!(
            "<g transform=\"scale(.1)\"><g aria-hidden=\"true\" fill=\"{shadow}\">\
             <text x=\"{x}\" y=\"{sy}\" fill-opacity=\".8\" filter=\"url(#blur)\" textLength=\"{len}\">{content}</text>\
             <text x=\"{x}\" y=\"{sy}\" fill-opacity=\".3\" textLength=\"{len}\">{content}</text></g>\
             <text x=\"{x}\" y=\"{y}\" textLength=\"{len}\"{fill}>{content}</text></g>"
        )
    } else {
        format!("<text x=\"{x}\" y=\"{y}\" transform=\"scale(.1)\" textLength=\"{len}\"{fill}>{content}</text>")
    }
}

/// The bold for-the-badge message run.
pub(crate) fn bold_run(cx: f64, width: f64, text: &str, bg: &str) -> String {
    let (fg, _) = ink(bg);
    let fill = if fg == "#fff" {
        String::new()
    } else {
        format!(" fill=\"{fg}\"")
    };
    format!(
        "<text transform=\"scale(.1)\" x=\"{}\" y=\"175\" textLength=\"{}\" font-weight=\"bold\"{fill}>{}</text>",
        fmt(cx * 10.0),
        fmt(width * 10.0),
        esc(text)
    )
}

/// Social text run: white emboss under `#333` ink.
pub(crate) fn social_run(cx: f64, width: f64, text: &str) -> String {
    let (x, len, content) = (fmt(cx * 10.0), fmt(width * 10.0), esc(text));
    format!(
        "<text aria-hidden=\"true\" x=\"{x}\" y=\"150\" fill=\"#fff\" transform=\"scale(.1)\" textLength=\"{len}\">{content}</text>\
         <text x=\"{x}\" y=\"140\" transform=\"scale(.1)\" textLength=\"{len}\">{content}</text>"
    )
}

/// Shortest decimal for an SVG number (`12`, `12.5`, `12.25`).
pub(crate) fn fmt(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_match_badge_maker() {
        // shields: /badge/build-passing → label 27, message 41 text px.
        assert_eq!(measure("build", PillStyle::Flat, Part::Label, false), 27.0);
        assert_eq!(
            measure("passing", PillStyle::Flat, Part::Message, false),
            41.0
        );
        // for-the-badge: AGENT-READY 86.75 → 86 + 13.75; 92/100 bold → 42 + 7.5.
        assert_eq!(
            measure("AGENT-READY", PillStyle::ForTheBadge, Part::Label, false),
            86.75
        );
        assert_eq!(
            measure("92/100", PillStyle::ForTheBadge, Part::Message, false),
            49.5
        );
        // social measures bold Helvetica: "Stars" → 27, "1k" → 13.
        assert_eq!(
            measure("Stars", PillStyle::Social, Part::Label, false),
            27.0
        );
        assert_eq!(measure("1k", PillStyle::Social, Part::Message, false), 13.0);
        assert_eq!(measure("", PillStyle::Flat, Part::Label, false), 1.0);
        assert_eq!(measure("", PillStyle::ForTheBadge, Part::Label, false), 0.0);
    }

    #[test]
    fn styles_parse_and_carry_shields_boxes() {
        assert_eq!(PillStyle::parse("flat-square"), PillStyle::FlatSquare);
        assert_eq!(PillStyle::parse("FOR-THE-BADGE"), PillStyle::ForTheBadge);
        assert_eq!(PillStyle::parse("unknown"), PillStyle::Flat);
        assert_eq!(PillStyle::Plastic.chrome().height, 18);
        assert_eq!(PillStyle::ForTheBadge.chrome().height, 28);
        assert_eq!(PillStyle::FlatSquare.chrome().radius, 0.0);
        assert!(PillStyle::Flat.chrome().shadow && !PillStyle::FlatSquare.chrome().shadow);
    }

    #[test]
    fn ink_follows_badge_maker_brightness() {
        assert_eq!(ink("#44bb00").0, "#fff");
        assert_eq!(ink("#ffffff").0, "#333");
        assert_eq!(fmt(553.75), "553.75");
        assert_eq!(fmt(195.0), "195");
    }
}
