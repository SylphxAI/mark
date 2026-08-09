//! Pill application: pure MarkSpec → pill SVG (the atomic status mark).
//!
//! The pill is the smallest mark: geometry is auto-sized; paint comes from the
//! shared grammar (theme defines the palette, explicit color wins otherwise);
//! motion applies at text level (ambient is meaningless at this size).

use crate::capabilities::mark::domain::color::contrasting_fg;
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::svg::{ensure_hash, esc, svg_doc};
use crate::capabilities::mark::domain::theme;
use crate::capabilities::mark::domain::{
    cap_text, named_color, normalize_animation, normalize_hex_token, MarkSpec, PillStyle,
    MAX_LABEL_CHARS, MAX_MESSAGE_CHARS,
};

fn resolve_color(c: Option<&str>, fallback: &str) -> String {
    let Some(c) = c else {
        return fallback.to_string();
    };
    if let Some(n) = named_color(c) {
        return normalize_hex_token(n).unwrap_or_else(|| n.to_string());
    }
    normalize_hex_token(c).unwrap_or_else(|| fallback.to_string())
}

fn measure(text: &str, style: PillStyle) -> u32 {
    let unit = if style == PillStyle::ForTheBadge {
        7.2
    } else {
        6.5
    };
    let pad = if style == PillStyle::ForTheBadge {
        20.0
    } else {
        14.0
    };
    (text.chars().count() as f32 * unit + pad).ceil() as u32
}

/// Render a pill from resolved parts (shared by the deploy mark).
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_pill(
    label: &str,
    message: &str,
    color: Option<&str>,
    label_color: Option<&str>,
    style: PillStyle,
    theme_name: Option<&str>,
    animation: Option<&str>,
) -> String {
    let theme = theme_name.and_then(theme::get);

    let msg_color = if let Some(t) = theme {
        t.accent.to_string()
    } else {
        resolve_color(color, "4A90E2")
    };
    let lbl_color = if let Some(t) = theme {
        t.bg.to_string()
    } else {
        resolve_color(
            label_color,
            if style == PillStyle::Social {
                "FFFFFF"
            } else {
                "555555"
            }
        )
    };

    // Motion at pill scale is text-level only; ambient is static here.
    let anim = match normalize_animation(animation) {
        "ambient" | "none" => "none",
        a => a,
    };

    let label = cap_text(label, MAX_LABEL_CHARS);
    let message = if message.is_empty() {
        "ok".into()
    } else {
        cap_text(message, MAX_MESSAGE_CHARS)
    };

    let h: u32 = match style {
        PillStyle::ForTheBadge => 28,
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
        PillStyle::Pill | PillStyle::Social => h as f32 / 2.0,
        PillStyle::ForTheBadge => 4.0,
        _ => 3.0,
    };

    let label_fg = ensure_hash(&contrasting_fg(&lbl_color));
    let msg_fg = ensure_hash(&contrasting_fg(&msg_color));
    let font = if style == PillStyle::ForTheBadge {
        "font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"11\" font-weight=\"700\" letter-spacing=\"0.5\""
    } else {
        "font-family=\"Verdana,DejaVu Sans,sans-serif\" font-size=\"11\" font-weight=\"500\""
    };
    let ty = if style == PillStyle::ForTheBadge {
        18
    } else {
        14
    };
    let label_text = if style == PillStyle::ForTheBadge {
        label.to_uppercase()
    } else {
        label.clone()
    };
    let message_text = if style == PillStyle::ForTheBadge {
        message.to_uppercase()
    } else {
        message.clone()
    };

    let mut body = String::new();
    if style == PillStyle::Plastic {
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
    if style == PillStyle::Plastic {
        body.push_str(&format!(
            "<rect width=\"{w}\" height=\"{h}\" fill=\"url(#p)\"/>"
        ));
    }
    body.push_str("</g>");
    body.push_str(&format!(
        "<rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\" fill=\"none\" stroke=\"#000\" stroke-opacity=\".08\"/>"
    ));
    if !label.is_empty() {
        let open = text_open_attrs(anim, 0, w, h);
        let children = text_children(anim, 0, w, h);
        body.push_str(&format!(
            "<text x=\"{}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{label_fg}\" {font}{open}>{}{children}</text>",
            lw as f32 / 2.0,
            esc(&label_text)
        ));
    }
    let open = text_open_attrs(anim, 1, w, h);
    let children = text_children(anim, 1, w, h);
    body.push_str(&format!(
        "<text x=\"{}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{msg_fg}\" {font}{open}>{}{children}</text>",
        lw as f32 + mw as f32 / 2.0,
        esc(&message_text)
    ));

    svg_doc(w, h, &body)
}

/// Pill form entry: paint and text come from the shared MarkSpec grammar.
pub fn render(spec: &MarkSpec) -> String {
    render_pill(
        spec.pill.label.as_deref().unwrap_or(""),
        spec.pill.message.as_deref().unwrap_or("ok"),
        spec.color.as_deref(),
        spec.pill.label_color.as_deref(),
        PillStyle::parse(spec.pill.style.as_deref().unwrap_or("flat")),
        spec.theme.as_deref(),
        spec.animation.as_deref(),
    )
}
