//! Profile application: pure MarkSpec → name + tagline card SVG.
//!
//! The profile card is the identity successor (ADR-0004 / delivery authority):
//! the URL supplies the name (`text`) and tagline (`desc`). Retired `identity`
//! form ids parse here. Optional art background, any palette, text-level
//! motion, native width × height.

use crate::capabilities::mark::domain::color::{contrasting_fg, resolve_fill};
use crate::capabilities::mark::domain::motion::{text_children, text_open_attrs};
use crate::capabilities::mark::domain::shapes::{normalize_art_type, shape_background, shape_defs};
use crate::capabilities::mark::domain::svg::{credit_mark, ensure_hash, esc, monogram, svg_doc};
use crate::capabilities::mark::domain::{
    cap_text, normalize_animation, MarkSpec, MAX_DESC_CHARS, MAX_TEXT_CHARS,
};

pub fn render(spec: &MarkSpec) -> String {
    let name = cap_text(spec.text.as_deref().unwrap_or("Mark"), MAX_TEXT_CHARS);
    let tagline = cap_text(spec.desc.as_deref().unwrap_or(""), MAX_DESC_CHARS);
    let initials = monogram(&name);

    let w = spec.width.unwrap_or(640).clamp(280, 1200);
    let h = spec.height.unwrap_or(200).clamp(80, 400);
    let wf = w as f32;
    let hf = h as f32;

    let fill = resolve_fill(
        spec.color.as_deref(),
        spec.theme.as_deref(),
        &format!("profile-{name}"),
        "mg",
    );
    let art = spec
        .art
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))
        .map(normalize_art_type);

    let radius = (hf * 0.08).clamp(12.0, 20.0);
    let tile = (hf * 0.56).clamp(56.0, 96.0);
    let tile_x = (wf * 0.045).clamp(20.0, 36.0);
    let tile_y = (hf - tile) / 2.0;
    let tile_rx = (tile * 0.18).clamp(10.0, 18.0);
    let text_x = tile_x + tile + (wf * 0.035).clamp(16.0, 28.0);
    let name_size = (hf * 0.18).clamp(22.0, 36.0);
    let tag_size = (hf * 0.08).clamp(12.0, 16.0);
    let mono_size = (tile * 0.38).clamp(20.0, 36.0);
    let has_tag = !tagline.is_empty();
    let name_y = if has_tag { hf * 0.44 } else { hf * 0.52 };
    let tag_y = hf * 0.64;

    let font_family = match spec
        .font
        .as_deref()
        .map(|f| f.to_ascii_lowercase())
        .as_deref()
    {
        Some("mono") => "ui-monospace,SFMono-Regular,Menlo,Consolas,monospace",
        _ => "ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif",
    };

    let anim = normalize_animation(spec.animation.as_deref());
    let anim = if anim == "ambient" { "none" } else { anim };
    let name_open = text_open_attrs(anim, 0, w, h);
    let name_children = text_children(anim, 0, w, h);
    let tag_open = text_open_attrs(anim, 1, w, h);
    let tag_children = text_children(anim, 1, w, h);

    let field = if let Some(ty) = art {
        format!(
            "<clipPath id=\"pc\"><rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\"/></clipPath>\
             <g clip-path=\"url(#pc)\">{}</g>",
            shape_background(ty, w, h, &fill, "header", false, 0.0),
        )
    } else {
        format!(
            "<rect width=\"{w}\" height=\"{h}\" rx=\"{radius}\" fill=\"{}\"/>",
            fill.fill
        )
    };

    let art_defs = art.map(|ty| shape_defs(ty, 0.0, &fill)).unwrap_or_default();

    let right_pad = (wf * 0.04).clamp(16.0, 28.0) + radius;
    let text_max = (wf - text_x - right_pad).max(48.0);
    let name = fit_line(&name, text_max, name_size);
    let tagline = if has_tag {
        fit_line(&tagline, text_max, tag_size)
    } else {
        tagline
    };
    let muted = ensure_hash(&fill.fg);
    let accent = ensure_hash(&fill.accent);
    let warm = ensure_hash(&fill.warm);
    let mono_ink = ensure_hash(&contrasting_fg(&fill.accent));
    let tag_node = if has_tag {
        format!(
            "<text x=\"{text_x}\" y=\"{tag_y}\" font-family=\"{font_family}\" font-size=\"{tag_size}\" \
             font-weight=\"450\" fill=\"{muted}\" fill-opacity=\"0.72\"{tag_open}>{tagline}{tag_children}</text>",
            tagline = esc(&tagline),
        )
    } else {
        String::new()
    };

    let ink = fill.fg_hash();
    let body = format!(
        "<defs>{fill_defs}{art_defs}\
           <linearGradient id=\"pm\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">\
             <stop offset=\"0%\" stop-color=\"{accent}\" stop-opacity=\"0.95\"/>\
             <stop offset=\"100%\" stop-color=\"{warm}\" stop-opacity=\"0.72\"/>\
           </linearGradient>\
           <clipPath id=\"pt\"><rect x=\"{text_x}\" y=\"0\" width=\"{text_max}\" height=\"{h}\"/></clipPath>\
         </defs>\
         {field}\
         <rect x=\"{tile_x}\" y=\"{tile_y}\" width=\"{tile}\" height=\"{tile}\" rx=\"{tile_rx}\" \
           fill=\"url(#pm)\" stroke=\"{accent}\" stroke-opacity=\"0.7\" stroke-width=\"1.25\"/>\
         <text x=\"{mx}\" y=\"{my}\" text-anchor=\"middle\" dominant-baseline=\"middle\" \
           font-family=\"{font_family}\" font-weight=\"750\" font-size=\"{mono_size}\" \
           letter-spacing=\"-0.04em\" fill=\"{mono_ink}\">{initials}</text>\
         <g clip-path=\"url(#pt)\">\
         <text x=\"{text_x}\" y=\"{name_y}\" font-family=\"{font_family}\" font-size=\"{name_size}\" \
           font-weight=\"700\" letter-spacing=\"-0.02em\" fill=\"{ink}\"{name_open}>{name}{name_children}</text>\
         {tag_node}\
         </g>\
         {credit}",
        fill_defs = fill.defs,
        mx = tile_x + tile / 2.0,
        my = tile_y + tile / 2.0 + 1.0,
        name = esc(&name),
        initials = esc(&initials),
        credit = credit_mark(w, h, spec.credit),
    );

    svg_doc(w, h, &body)
}

fn fit_line(text: &str, max_px: f32, font_size: f32) -> String {
    if line_advance(text, font_size) <= max_px {
        return text.to_string();
    }
    let ellipsis = '\u{2026}';
    let budget = (max_px - glyph_advance(ellipsis, font_size)).max(0.0);
    let mut used = 0.0;
    let mut out = String::new();
    for ch in text.chars() {
        let adv = glyph_advance(ch, font_size);
        if used + adv > budget {
            break;
        }
        out.push(ch);
        used += adv;
    }
    out.push(ellipsis);
    out
}

/// Proportional sans advances so wide glyphs (`W`, CJK) cannot outrun a 0.55em cap.
fn glyph_advance(ch: char, font_size: f32) -> f32 {
    let unit = match ch {
        ' ' | '\u{00A0}' => 0.30,
        'i' | 'l' | 'I' | 'j' | 't' | 'f' | 'r' | '|' | '\'' | '`' | '!' | '.' | ',' | ':'
        | ';' => 0.34,
        'm' | 'w' | 'M' | 'W' | '@' | '%' => 0.95,
        '1' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' => 0.40,
        c if c.is_ascii_uppercase() => 0.70,
        c if c.is_ascii_digit() => 0.58,
        c if !c.is_ascii() && c.is_alphanumeric() => 1.05,
        _ => 0.60,
    };
    // Bold name weight plus sidebearings run wider than a regular table.
    font_size * unit * 1.12
}

fn line_advance(line: &str, font_size: f32) -> f32 {
    line.chars().map(|c| glyph_advance(c, font_size)).sum()
}
