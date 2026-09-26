//! Profile application: pure MarkSpec → name + tagline card SVG.
//!
//! The profile card is the identity successor (ADR-0004 / delivery authority):
//! the URL supplies the name (`text`) and tagline (`desc`). Retired `identity`
//! form ids parse here. Optional art background, any palette, text-level
//! motion, native width × height.

use crate::capabilities::mark::domain::art::Art;
use crate::capabilities::mark::domain::color::{ink_over, mix, resolve_palette};
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{card_radius, stage};
use crate::capabilities::mark::domain::svg::{credit_mark, esc, svg_doc};
use crate::capabilities::mark::domain::text::{content_family, fit_line, monogram, Metric};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, MAX_DESC_CHARS, MAX_TEXT_CHARS,
};

pub fn render(spec: &MarkSpec) -> String {
    let name = cap_text(spec.text.as_deref().unwrap_or("Mark"), MAX_TEXT_CHARS);
    let tagline = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);
    let initials = monogram(&name);

    let w = spec.width.unwrap_or(640).clamp(280, 1200);
    let h = spec.height.unwrap_or(200).clamp(80, 400);
    let (wf, hf) = (w as f32, h as f32);

    let p = resolve_palette(
        spec.color.as_deref(),
        spec.theme.as_deref(),
        &format!("profile-{name}"),
    );
    let art = spec
        .art
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))
        .map(Art::parse)
        // Terminal and transparent are banner compositions, not card fields.
        .filter(|a| !matches!(a, Art::Terminal | Art::Transparent));

    let rx = card_radius(hf);
    let [a1, a2, _] = &p.accents;
    let (field_defs, field) = match art {
        Some(a) => {
            let st = stage(a, w, h, &p, false);
            (st.defs, st.back)
        }
        None => {
            // A calm card: the base with two faint pools of accent light.
            let o = if p.light { 0.16 } else { 0.28 };
            (
                format!(
                    "<radialGradient id=\"pg1\" cx=\"0.92\" cy=\"0\" r=\"0.8\">\
                     <stop offset=\"0\" stop-color=\"{a1}\" stop-opacity=\"{o}\"/>\
                     <stop offset=\"1\" stop-color=\"{a1}\" stop-opacity=\"0\"/></radialGradient>\
                     <radialGradient id=\"pg2\" cx=\"0.05\" cy=\"1\" r=\"0.7\">\
                     <stop offset=\"0\" stop-color=\"{a2}\" stop-opacity=\"{o2:.2}\"/>\
                     <stop offset=\"1\" stop-color=\"{a2}\" stop-opacity=\"0\"/></radialGradient>",
                    o2 = o * 0.55,
                ),
                format!(
                    "<rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"{bg}\"/>\
                     <rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"url(#pg1)\"/>\
                     <rect width=\"{w}\" height=\"{h}\" rx=\"{rx}\" fill=\"url(#pg2)\"/>\
                     <rect x=\"0.5\" y=\"0.5\" width=\"{bw}\" height=\"{bh}\" rx=\"{rx2}\" fill=\"none\" \
                     stroke=\"{ink}\" stroke-opacity=\"0.09\"/>",
                    bg = p.bg,
                    ink = p.ink,
                    bw = wf - 1.0,
                    bh = hf - 1.0,
                    rx2 = rx - 0.5,
                ),
            )
        }
    };
    let ink = match art {
        Some(a) => stage(a, w, h, &p, false).ink.unwrap_or(p.ink.clone()),
        None => p.ink.clone(),
    };

    let tile = (hf * 0.5).clamp(48.0, 112.0).round();
    let pad = (hf * 0.16).clamp(20.0, 40.0).round();
    let tile_y = ((hf - tile) / 2.0).round();
    let tile_rx = (tile * 0.24).round();
    let text_x = pad + tile + (hf * 0.12).clamp(16.0, 28.0).round();
    let name_size = (hf * 0.16).clamp(20.0, 40.0).round();
    let tag_size = (hf * 0.075).clamp(12.0, 18.0).round();
    let mono_size = (tile * 0.36).round();
    let has_tag = !tagline.is_empty();
    let gap = if has_tag {
        name_size * 0.55 + tag_size * 0.95
    } else {
        0.0
    };
    let name_y = hf / 2.0 - gap / 2.0;
    let tag_y = hf / 2.0 + gap / 2.0;

    let family = content_family(spec.font.as_deref());
    let anim = match normalize_animation(spec.animation.as_deref()) {
        "fade" => "fade",
        "rise" | "type" => "rise",
        _ => "none",
    };
    let text_max = (wf - text_x - pad).max(48.0);
    let name = fit_line(&name, text_max, name_size, Metric::Bold);
    let tagline = fit_line(&tagline, text_max, tag_size, Metric::Display);
    let tile_mid = mix(a1, a2, 0.5);
    let mono_ink = ink_over(&tile_mid);
    let tag_node = if has_tag {
        format!(
            "<text x=\"{text_x}\" y=\"{tag_y:.1}\" dominant-baseline=\"central\" font-family=\"{family}\" \
             font-size=\"{tag_size}\" font-weight=\"400\" fill=\"{ink}\" fill-opacity=\"0.7\"{open}>{t}{kids}</text>",
            t = esc(&tagline),
            open = text_open_attrs(anim, 1, w, h),
            kids = text_children(anim, 1, w, h),
        )
    } else {
        String::new()
    };

    let body = format!(
        "<defs>{field_defs}\
           <linearGradient id=\"pm\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\">\
             <stop offset=\"0\" stop-color=\"{a1}\"/><stop offset=\"1\" stop-color=\"{a2}\"/>\
           </linearGradient>\
           <clipPath id=\"pt\"><rect x=\"{text_x}\" width=\"{text_max}\" height=\"{h}\"/></clipPath>\
         </defs>\
         {field}\
         <rect x=\"{pad}\" y=\"{tile_y}\" width=\"{tile}\" height=\"{tile}\" rx=\"{tile_rx}\" fill=\"url(#pm)\"/>\
         <rect x=\"{pi}\" y=\"{ti}\" width=\"{ts}\" height=\"{ts}\" rx=\"{tri}\" fill=\"none\" \
           stroke=\"#FFFFFF\" stroke-opacity=\"0.22\"/>\
         <text x=\"{mx}\" y=\"{my}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
           font-family=\"{family}\" font-weight=\"700\" font-size=\"{mono_size}\" \
           letter-spacing=\"-0.02em\" fill=\"{mono_ink}\">{initials}</text>\
         <g clip-path=\"url(#pt)\">\
         <text x=\"{text_x}\" y=\"{name_y:.1}\" dominant-baseline=\"central\" font-family=\"{family}\" \
           font-size=\"{name_size}\" font-weight=\"700\" letter-spacing=\"-0.02em\" fill=\"{ink}\"{nopen}>\
           {name}{nkids}</text>\
         {tag_node}</g>\
         {credit}",
        pi = pad + 0.5,
        ti = tile_y + 0.5,
        ts = tile - 1.0,
        tri = tile_rx - 0.5,
        mx = pad + tile / 2.0,
        my = tile_y + tile / 2.0,
        name = esc(&name),
        initials = esc(&initials),
        nopen = text_open_attrs(anim, 0, w, h),
        nkids = text_children(anim, 0, w, h),
        credit = credit_mark(w, h, spec.credit),
    );

    svg_doc(w, h, &body)
}
