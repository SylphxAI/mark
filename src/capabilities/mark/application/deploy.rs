//! Deploy application: conversion mark ("deployed on Sylphx").
//!
//! Same grammar as the pill (style / theme / color / motion / font) with a
//! left mark tile so the conversion surface is not a generic shields badge.

use crate::capabilities::mark::domain::color::contrasting_fg;
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::svg::{ensure_hash, esc, svg_doc};
use crate::capabilities::mark::domain::theme;
use crate::capabilities::mark::domain::{
    cap_text, named_color, normalize_animation, normalize_hex_token, MarkSpec, PillStyle,
    MAX_SERVICE_CHARS,
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
        resolve_color(spec.color.as_deref(), "D87000")
    };
    let lbl_color = if let Some(t) = theme {
        t.bg.to_string()
    } else {
        resolve_color(Some("1A1A2E"), "1A1A2E")
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

    let h: u32 = match style {
        PillStyle::ForTheBadge => 28,
        _ => 20,
    };
    let tile = h;
    let lw = measure(&label_text, style);
    let mw = measure(&message_text, style);
    let w = tile + lw + mw;
    let radius = match style {
        PillStyle::Pill | PillStyle::Social => h as f32 / 2.0,
        PillStyle::ForTheBadge => 4.0,
        _ => 3.0,
    };
    let inset = (h as f32 * 0.18).clamp(3.0, 5.0);
    let inner = h as f32 - inset * 2.0;
    let mark_cx = tile as f32 / 2.0;
    let mark_cy = h as f32 / 2.0;
    let mark_r = inner * 0.22;

    let family = match spec.font.as_deref().map(|f| f.to_ascii_lowercase()).as_deref() {
        Some("mono") => "ui-monospace,SFMono-Regular,Menlo,Consolas,monospace",
        _ => "ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif",
    };
    let font = if style == PillStyle::ForTheBadge {
        format!("font-family=\"{family}\" font-size=\"11\" font-weight=\"700\" letter-spacing=\"0.5\"")
    } else {
        format!("font-family=\"{family}\" font-size=\"11\" font-weight=\"500\"")
    };
    let ty = if style == PillStyle::ForTheBadge { 18 } else { 14 };

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
