//! Deploy application: conversion mark ("deployed on Sylphx").
//!
//! Same grammar as the pill (style / theme / color / motion / font) with a
//! left mark tile so the conversion surface is not a generic shields badge.

use crate::capabilities::mark::domain::color::{contrasting_fg, resolve_paint};
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::pill::{measure, PillMetrics};
use crate::capabilities::mark::domain::svg::{ensure_hash, esc, svg_doc};
use crate::capabilities::mark::domain::text::content_family;
use crate::capabilities::mark::domain::theme;
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, PillStyle, MAX_SERVICE_CHARS,
};

pub fn render(spec: &MarkSpec) -> String {
    let service = cap_text(
        spec.deploy.service.as_deref().unwrap_or("Sylphx"),
        MAX_SERVICE_CHARS,
    );
    let message = if service.is_empty() {
        "Sylphx".into()
    } else {
        format!("{service} · Sylphx")
    };
    let label = "deployed on";
    let style = PillStyle::parse(spec.pill.style.as_deref().unwrap_or("flat"));
    let theme = spec.theme.as_deref().and_then(theme::get);

    let msg_color = if let Some(t) = theme {
        t.accent.to_string()
    } else {
        resolve_paint(spec.color.as_deref(), "D87000")
    };
    let lbl_color = if let Some(t) = theme {
        t.bg.to_string()
    } else {
        resolve_paint(Some("1A1A2E"), "1A1A2E")
    };

    let anim = match normalize_animation(spec.animation.as_deref()) {
        "ambient" | "none" => "none",
        a => a,
    };

    let label_text = if style == PillStyle::ForTheBadge {
        label.to_uppercase()
    } else {
        label.to_string()
    };
    let message_text = if style == PillStyle::ForTheBadge {
        message.to_uppercase()
    } else {
        message
    };

    let metrics = PillMetrics::new(style, content_family(spec.font.as_deref()));
    let h = metrics.height;
    let tile = h;
    let lw = measure(&label_text, style);
    let mw = measure(&message_text, style);
    let w = tile + lw + mw;
    let radius = metrics.radius;
    let inset = (h as f32 * 0.18).clamp(3.0, 5.0);
    let inner = h as f32 - inset * 2.0;
    let mark_cx = tile as f32 / 2.0;
    let mark_cy = h as f32 / 2.0;
    let mark_r = inner * 0.22;

    let font = &metrics.text_attrs;
    let ty = metrics.baseline;

    let label_fg = ensure_hash(&contrasting_fg(&lbl_color));
    let msg_fg = ensure_hash(&contrasting_fg(&msg_color));
    let tile_fill = ensure_hash(&msg_color);
    let tile_ink = ensure_hash(&contrasting_fg(&msg_color));
    let lbl = ensure_hash(&lbl_color);
    let msg = ensure_hash(&msg_color);

    let label_open = text_open_attrs(anim, 0, w, h);
    let label_children = text_children(anim, 0, w, h);
    let msg_open = text_open_attrs(anim, 1, w, h);
    let msg_children = text_children(anim, 1, w, h);

    let body = format!(
        "<clipPath id=\"d\"><rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\"/></clipPath>\
         <g clip-path=\"url(#d)\">\
           <rect width=\"{tile}\" height=\"{h}\" fill=\"{tile_fill}\"/>\
           <rect x=\"{tile}\" width=\"{lw}\" height=\"{h}\" fill=\"{lbl}\"/>\
           <rect x=\"{mid}\" width=\"{mw}\" height=\"{h}\" fill=\"{msg}\"/>\
         </g>\
         <rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\" fill=\"none\" stroke=\"#000\" stroke-opacity=\".08\"/>\
         <circle cx=\"{mark_cx}\" cy=\"{mark_cy}\" r=\"{mark_r}\" fill=\"{tile_ink}\"/>\
         <circle cx=\"{mark_cx}\" cy=\"{mark_cy}\" r=\"{ring}\" fill=\"none\" stroke=\"{tile_ink}\" stroke-width=\"1.25\"/>\
         <text x=\"{lx}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{label_fg}\" {font}{label_open}>{label}{label_children}</text>\
         <text x=\"{mx}\" y=\"{ty}\" text-anchor=\"middle\" fill=\"{msg_fg}\" {font}{msg_open}>{message}{msg_children}</text>",
        mid = tile + lw,
        ring = mark_r + 3.0,
        lx = tile as f32 + lw as f32 / 2.0,
        mx = tile as f32 + lw as f32 + mw as f32 / 2.0,
        label = esc(&label_text),
        message = esc(&message_text),
    );

    svg_doc(w, h, &body)
}
