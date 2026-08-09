//! Identity application: pure MarkSpec → fleet brand card SVG.
//!
//! The identity mark composes the full grammar: fleet brand vocabulary, any
//! palette/theme, optional art background, text-level motion, and width
//! scaling (identity at any size).

use crate::capabilities::mark::domain::color::resolve_fill;
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{normalize_art_type, shape_background, shape_defs};
use crate::capabilities::mark::domain::svg::{credit_mark, esc, svg_doc};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, MAX_BRAND_CHARS, MAX_TAGLINE_CHARS,
};

fn brand_identity(raw: &str) -> (String, &'static str, &'static str) {
    let key = raw.to_ascii_lowercase();
    match key.as_str() {
        "sylphx" | "sylphxai" => ("Sylphx".into(), "sylphx", "AI-native platform for developers"),
        "cubeage" => ("Cubeage".into(), "cubeage", "Games & entertainment"),
        "epiow" | "epiowai" => ("Epiow".into(), "epiow", "B2B technology"),
        "ozyrix" | "ozyrixltd" => ("Ozyrix".into(), "ozyrix", "Premium tech accessories"),
        "kyle" | "shtse8" => ("Kyle Tse".into(), "kyle", "Builder · multi-company portfolio"),
        other => (cap_text(other, MAX_BRAND_CHARS), "dark", "Brand mark"),
    }
}

pub fn render(spec: &MarkSpec) -> String {
    let (name, theme_key, default_tag) =
        brand_identity(spec.identity.brand.as_deref().unwrap_or("sylphx"));
    let tagline = cap_text(
        spec.identity.tagline.as_deref().unwrap_or(default_tag),
        MAX_TAGLINE_CHARS,
    );

    let w = spec.width.unwrap_or(640).clamp(280, 1200);
    let scale = w as f32 / 640.0;
    let h = 200u32;

    let theme = spec.theme.as_deref().or(Some(theme_key));
    let fill = resolve_fill(spec.color.as_deref(), theme, &format!("identity-{name}"), "id");
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

    let anim = normalize_animation(spec.animation.as_deref());
    let anim = if anim == "ambient" { "none" } else { anim };

    let name_open = text_open_attrs(anim, 0, 640, h);
    let name_children = text_children(anim, 0, 640, h);
    let tag_open = text_open_attrs(anim, 1, 640, h);
    let tag_children = text_children(anim, 1, 640, h);

    let body = format!(
        "{bg}\
         <circle cx=\"72\" cy=\"100\" r=\"40\" fill=\"{accent}\" fill-opacity=\"0.9\"/>\
         <circle cx=\"72\" cy=\"100\" r=\"16\" fill=\"{fg}\" fill-opacity=\"0.95\"/>\
         <text x=\"140\" y=\"88\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"36\" font-weight=\"700\" fill=\"{fg}\"{name_open}>{name}{name_children}</text>\
         <text x=\"140\" y=\"122\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"16\" fill=\"{muted}\"{tag_open}>{tagline}{tag_children}</text>\
         <text x=\"140\" y=\"158\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{fg}\" fill-opacity=\"0.55\">mark · brand kit</text>\
         {credit}",
        accent = fill.accent,
        fg = fill.fg_hash(),
        muted = fill.muted,
        name = esc(&name),
        name_children = {name_children},
        tagline = esc(&tagline),
        tag_children = {tag_children},
        credit = credit_mark(640, h, spec.credit),
    );

    if (scale - 1.0).abs() < 0.001 {
        svg_doc(w, h, &body)
    } else {
        svg_doc(w, h, &format!("<g transform=\"scale({scale})\">{body}</g>"))
    }
}
