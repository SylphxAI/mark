//! The social style: badge-maker's GitHub-button look (light box + count
//! bubble). Colors are fixed by the style, as on shields.

use super::badge::{ring, Badge};
use crate::capabilities::mark::domain::pill::{fmt, measure, social_run, text_group_open, Part};
use crate::capabilities::mark::domain::svg::svg_doc;
use crate::capabilities::mark::domain::PillStyle;

const DEFS: &str = "<linearGradient id=\"a\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#fcfcfc\" stop-opacity=\"0\"/>\
<stop offset=\"1\" stop-opacity=\".1\"/></linearGradient>";

/// Social label is styled with a leading capital (measured capitalised).
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

pub(super) fn social(b: &Badge) -> String {
    const GUTTER: f64 = 6.0;
    let st = PillStyle::Social;
    let label = capitalize(&b.label);
    let label_tw = measure(&label, st, Part::Label, b.mono);
    let msg_tw = measure(&b.message, st, Part::Message, b.mono);
    let logo_w = b
        .logo
        .as_ref()
        .map(|l| l.width() + b.logo_pad())
        .unwrap_or(0.0);
    let ring_slot = b.ring.map(|_| st.ring_diameter() + 3.0).unwrap_or(0.0);
    let label_rect_w = label_tw + logo_w + 10.0;
    let msg_rect_w = msg_tw + 8.0 + ring_slot;
    let has_msg = !b.message.is_empty();
    let w = label_rect_w + 1.0 + if has_msg { GUTTER + msg_rect_w } else { 0.0 };
    let (lw, mw) = (fmt(label_rect_w), fmt(msg_rect_w));

    let mut body = b.title(&label, &b.message);
    body.push_str(DEFS);
    body.push_str(&format!(
        "<g stroke=\"#d5d5d5\"><rect stroke=\"none\" fill=\"#fcfcfc\" x=\".5\" y=\".5\" width=\"{lw}\" height=\"19\" rx=\"2\"/>"
    ));
    let bubble_x = label_rect_w + GUTTER;
    if has_msg {
        let main_x = fmt(bubble_x + 0.5);
        body.push_str(&format!(
            "<rect x=\"{main_x}\" y=\".5\" width=\"{mw}\" height=\"19\" rx=\"2\" fill=\"#fafafa\"/>\
             <rect x=\"{}\" y=\"7.5\" width=\".5\" height=\"5\" stroke=\"#fafafa\"/>\
             <path d=\"M{main_x} 6.5 l-3 3v1 l3 3\" fill=\"#fafafa\"/>",
            fmt(bubble_x)
        ));
    }
    body.push_str("</g>");
    if let Some(logo) = &b.logo {
        body.push_str(&logo.paint(5.0, 20.0));
    }
    if let (Some(frac), true) = (b.ring, has_msg) {
        let d = st.ring_diameter();
        body.push_str(&ring(bubble_x + 4.5, 10.0, d, frac, &b.color, "#e1e4e8"));
    }
    body.push_str(&text_group_open(st, b.mono));
    body.push_str(&format!(
        "<rect stroke=\"#d5d5d5\" fill=\"url(#a)\" x=\".5\" y=\".5\" width=\"{lw}\" height=\"19\" rx=\"2\"/>"
    ));
    let label_cx = logo_w + label_tw / 2.0 + 5.0;
    body.push_str(&social_run(label_cx, label_tw, &label));
    if has_msg {
        let cx = bubble_x + ring_slot + 4.0 + msg_tw / 2.0;
        body.push_str(&social_run(cx, msg_tw, &b.message));
    }
    body.push_str("</g>");
    svg_doc(fmt(w), 20, &body)
}
