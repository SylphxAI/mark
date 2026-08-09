//! Profile application: pure MarkSpec → profile card SVG.
//!
//! The profile card is text-driven (ADR-0004): the URL supplies the name
//! (`text`) and tagline (`desc`) — no baked identities, no brand tables, no
//! personal or company names in the product. Optional art background, any
//! palette, text-level motion, width scaling.

use crate::capabilities::mark::domain::color::resolve_fill;
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{normalize_art_type, shape_background, shape_defs};
use crate::capabilities::mark::domain::svg::{credit_mark, esc, svg_doc};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, MAX_DESC_CHARS, MAX_TEXT_CHARS,
};


pub fn render(spec: &MarkSpec) -> String {
    let name = cap_text(spec.text.as_deref().unwrap_or("Mark"), MAX_TEXT_CHARS);
    let tagline = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);

    let w = spec.width.unwrap_or(640).clamp(280, 1200);
    let scale = w as f32 / 640.0;
    let h = 200u32;

    let fill = resolve_fill(
        spec.color.as_deref(),
        spec.theme.as_deref(),
        &format!("profile-{name}"),
        "id",
    );
    let art = spec.art.as_deref().map(normalize_art_type);

    let bg = if let Some(ty) = art {
        format!(
            "<defs>{}{}</defs>{}{}",
            fill.defs,
            shape_defs(ty, 0.0, &fill),
            shape_background(ty, 640, h, &fill, "header", false, 0.0),
            ""
        )
    } else {
        format!(
            "<defs>{}</defs><rect width=\"640\" height=\"{h}\" rx=\"16\" fill=\"{}\"/>",
            fill.defs, fill.fill
        )
    };

    let font_family = match spec.font.as_deref().map(|f| f.to_ascii_lowercase()).as_deref() {
        Some("mono") => "ui-monospace,SFMono-Regular,Menlo,Consolas,monospace",
        _ => "ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif",
    };

    let anim = normalize_animation(spec.animation.as_deref());
    let anim = if anim == "ambient" { "none" } else { anim };

    let name_open = text_open_attrs(anim, 0, 640, h);
    let name_children = text_children(anim, 0, 640, h);
    let tag_open = text_open_attrs(anim, 1, 640, h);
    let tag_children = text_children(anim, 1, 640, h);
    let tag_node = if tagline.is_empty() {
        String::new()
    } else {
        format!(
            "<text x=\"140\" y=\"122\" font-family=\"{font_family}\" font-size=\"16\" fill=\"{muted}\"{tag_open}>{tagline}{tag_children}</text>",
            muted = fill.muted,
            tagline = esc(&tagline),
            tag_children = tag_children,
        )
    };

    let body = format!(
        "{bg}\
         <circle cx=\"72\" cy=\"100\" r=\"40\" fill=\"{accent}\" fill-opacity=\"0.9\"/>\
         <circle cx=\"72\" cy=\"100\" r=\"16\" fill=\"{fg}\" fill-opacity=\"0.95\"/>\
         <text x=\"140\" y=\"88\" font-family=\"{font_family}\" font-size=\"36\" font-weight=\"700\" fill=\"{fg}\"{name_open}>{name}{name_children}</text>\
         {tag_node}\
         {credit}",
        accent = fill.accent,
        fg = fill.fg_hash(),
        name = esc(&name),
        name_open = name_open,
        name_children = name_children,
        credit = credit_mark(640, h, spec.credit),
    );

    if (scale - 1.0).abs() < 0.001 {
        svg_doc(w, h, &body)
    } else {
        svg_doc(w, h, &format!("<g transform=\"scale({scale})\">{body}</g>"))
    }
}
