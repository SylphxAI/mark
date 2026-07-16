//! Shields-like badges.

use crate::color::contrasting_fg;
use crate::svg::{ensure_hash, esc, is_hex_color, strip_hash, svg_doc};
use crate::themes;

#[derive(Debug, Clone)]
pub struct BadgeInput {
    pub label: Option<String>,
    pub message: String,
    pub color: Option<String>,
    pub label_color: Option<String>,
    pub style: BadgeStyle,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeStyle {
    #[default]
    Flat,
    Plastic,
    ForTheBadge,
    Social,
    Pill,
}

impl BadgeStyle {
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

fn named_color(c: &str) -> Option<&'static str> {
    Some(match c.to_ascii_lowercase().as_str() {
        "brightgreen" => "4C1",
        "green" => "97CA00",
        "yellow" => "DFB317",
        "yellowgreen" => "A4A61D",
        "orange" => "FE7D37",
        "red" => "E05D44",
        "blue" => "007EC6",
        "lightgrey" | "lightgray" => "9F9F9F",
        "success" => "27AE60",
        "important" => "FE7D37",
        "critical" => "E05D44",
        "informational" => "007EC6",
        "inactive" => "9F9F9F",
        "sylphx" => "D87000",
        "cubeage" => "E03840",
        "epiow" => "7C3AED",
        "ozyrix" => "C9A227",
        _ => return None,
    })
}

fn resolve_color(c: Option<&str>, fallback: &str) -> String {
    let Some(c) = c else {
        return fallback.to_string();
    };
    if let Some(n) = named_color(c) {
        return expand3(n);
    }
    let h = strip_hash(c);
    if is_hex_color(h) {
        return expand3(h);
    }
    fallback.to_string()
}

fn expand3(h: &str) -> String {
    let h = strip_hash(h);
    if h.len() == 3 {
        h.chars().flat_map(|c| [c, c]).collect()
    } else {
        h.chars().take(6).collect()
    }
}

fn measure(text: &str, style: BadgeStyle) -> u32 {
    let unit = if style == BadgeStyle::ForTheBadge {
        7.2
    } else {
        6.5
    };
    let pad = if style == BadgeStyle::ForTheBadge {
        20.0
    } else {
        14.0
    };
    (text.chars().count() as f32 * unit + pad).ceil() as u32
}

pub fn render(input: &BadgeInput) -> String {
    let style = input.style;
    let theme = input.theme.as_deref().and_then(themes::get);

    let msg_color = if let Some(t) = theme {
        t.accent.to_string()
    } else {
        resolve_color(input.color.as_deref(), "4A90E2")
    };
    let lbl_color = if let Some(t) = theme {
        t.bg.to_string()
    } else {
        resolve_color(
            input.label_color.as_deref(),
            if style == BadgeStyle::Social {
                "FFFFFF"
            } else {
                "555555"
            },
        )
    };

    let label = input.label.clone().unwrap_or_default();
    let message = if input.message.is_empty() {
        "ok".into()
    } else {
        input.message.clone()
    };

    let h: u32 = match style {
        BadgeStyle::ForTheBadge => 28,
        _ => 20,
    };
    let lw = if label.is_empty() {
        0
    } else {
        measure(&label, style)
    };
    let mw = measure(&message, style);
    let w = (lw + mw).max(30);
    let radius = match style {
        BadgeStyle::Pill | BadgeStyle::Social => h as f32 / 2.0,
        BadgeStyle::ForTheBadge => 4.0,
        _ => 3.0,
    };

    let label_fg = ensure_hash(&contrasting_fg(&lbl_color));
    let msg_fg = ensure_hash(&contrasting_fg(&msg_color));
    let font = if style == BadgeStyle::ForTheBadge {
        "font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"11\" font-weight=\"700\" letter-spacing=\"0.5\""
    } else {
        "font-family=\"Verdana,DejaVu Sans,sans-serif\" font-size=\"11\" font-weight=\"500\""
    };
    let ty = if style == BadgeStyle::ForTheBadge {
        18
    } else {
        14
    };
    let label_text = if style == BadgeStyle::ForTheBadge {
        label.to_uppercase()
    } else {
        label.clone()
    };
    let message_text = if style == BadgeStyle::ForTheBadge {
        message.to_uppercase()
    } else {
        message.clone()
    };

    let mut body = String::new();
    if style == BadgeStyle::Plastic {
        body.push_str(
            "<defs><linearGradient id=\"p\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\
             <stop offset=\"0\" stop-color=\"#fff\" stop-opacity=\".7\"/>\
             <stop offset=\".1\" stop-color=\"#fff\" stop-opacity=\".1\"/>\
             <stop offset=\".9\" stop-opacity=\".3\"/>\
             <stop offset=\"1\" stop-opacity=\".5\"/></linearGradient></defs>",
        );
    }
    body.push_str(&format!(
        "<clipPath id=\"r\"><rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\"/></clipPath><g clip-path=\"url(#r)\">"
    ));
    if !label.is_empty() {
        body.push_str(&format!(
            "<rect width=\"{lw}\" height=\"{h}\" fill=\"{}\"/>",
            ensure_hash(&lbl_color)
        ));
    }
    body.push_str(&format!(
        "<rect x=\"{lw}\" width=\"{mw}\" height=\"{h}\" fill=\"{}\"/>",
        ensure_hash(&msg_color)
    ));
    if style == BadgeStyle::Plastic {
        body.push_str(&format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#p)\"/>"
        ));
    }
    body.push_str("</g>");
    body.push_str(&format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\" fill=\"none\" stroke=\"#000\" stroke-opacity=\".08\"/>"
    ));
    if !label.is_empty() {
        body.push_str(&format!(
            "<text x=\"{}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{label_fg}\" {font}>{}</text>",
            lw as f32 / 2.0,
            esc(&label_text)
        ));
    }
    body.push_str(&format!(
        "<text x=\"{}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{msg_fg}\" {font}>{}</text>",
        lw as f32 + mw as f32 / 2.0,
        esc(&message_text)
    ));

    svg_doc(w, h, &body)
}
