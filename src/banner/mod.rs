//! Banner / hero SVG renderer.

mod motion;
mod shapes;

pub use motion::ANIMATIONS;
pub use shapes::{is_banner_type, normalize_type, BANNER_TYPES, FEATURED_TYPES};

use crate::color::resolve_fill;
use crate::svg::{credit_mark, ensure_hash, esc, svg_doc};
use motion::{ambient_gain, normalize_animation, text_children, text_open_attrs};
use shapes::{shape_background, shape_defs};

#[derive(Debug, Clone, Default)]
pub struct BannerInput {
    pub type_name: Option<String>,
    pub color: Option<String>,
    pub theme: Option<String>,
    pub section: Option<String>,
    pub reversal: bool,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub text: Option<String>,
    pub desc: Option<String>,
    pub font_size: Option<u32>,
    pub desc_size: Option<u32>,
    pub font_color: Option<String>,
    pub font_align: Option<f32>,
    pub font_align_y: Option<f32>,
    pub desc_align: Option<f32>,
    pub desc_align_y: Option<f32>,
    pub rotate: Option<f32>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub text_bg: bool,
    pub animation: Option<String>,
    pub credit: bool,
    pub seed: Option<String>,
}

pub fn render(input: &BannerInput) -> String {
    let ty = normalize_type(input.type_name.as_deref().unwrap_or("aurora"));
    let height = input.height.unwrap_or(220).clamp(40, 600);
    let width = input.width.unwrap_or(880).clamp(200, 1400);
    let section = if input.section.as_deref() == Some("footer") {
        "footer"
    } else {
        "header"
    };
    let anim = normalize_animation(input.animation.as_deref());
    let gain = ambient_gain(anim);

    let seed = input
        .seed
        .clone()
        .unwrap_or_else(|| format!("{ty}-{}", input.text.as_deref().unwrap_or("")));
    let fill = resolve_fill(input.color.as_deref(), input.theme.as_deref(), &seed, "mg");

    let font_color = input
        .font_color
        .as_deref()
        .map(|c| ensure_hash(c.trim_start_matches('#')))
        .unwrap_or_else(|| ensure_hash(&fill.fg));

    let text = input.text.as_deref().unwrap_or("");
    let desc = input.desc.as_deref().unwrap_or("");
    let font_size = input
        .font_size
        .unwrap_or(if text.is_empty() { 40 } else { 48 })
        .clamp(10, 120);
    let desc_size = input.desc_size.unwrap_or(18).clamp(8, 60);
    let font_align = input.font_align.unwrap_or(50.0).clamp(0.0, 100.0);
    let font_align_y = input
        .font_align_y
        .unwrap_or(if desc.is_empty() { 50.0 } else { 44.0 })
        .clamp(0.0, 100.0);
    let desc_align = input.desc_align.unwrap_or(50.0).clamp(0.0, 100.0);
    let desc_align_y = input.desc_align_y.unwrap_or(68.0).clamp(0.0, 100.0);
    let rotate = input.rotate.unwrap_or(0.0);
    let stroke = input
        .stroke
        .as_deref()
        .map(|s| ensure_hash(s.trim_start_matches('#')));
    let stroke_width = input
        .stroke_width
        .unwrap_or(if stroke.is_some() { 1.0 } else { 0.0 });

    let lines: Vec<&str> = text.split('\n').filter(|l| !l.is_empty()).collect();
    let mut text_nodes = String::new();
    let n = lines.len().max(1) as f32;
    for (i, line) in lines.iter().enumerate() {
        let dy = (i as f32 - (n - 1.0) / 2.0) * font_size as f32 * 1.15;
        let x = width as f32 * font_align / 100.0;
        let y = height as f32 * font_align_y / 100.0 + dy;
        if input.text_bg {
            let bw = (line.chars().count() as f32 * font_size as f32 * 0.56).max(40.0);
            text_nodes.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{bw}\" height=\"{}\" rx=\"8\" fill=\"#000000\" fill-opacity=\"0.25\"/>",
                x - bw / 2.0,
                y - font_size as f32 * 0.8,
                font_size as f32 * 1.15
            ));
        }
        let stroke_attr = if let Some(ref s) = stroke {
            format!(" stroke=\"{s}\" stroke-width=\"{stroke_width}\" paint-order=\"stroke\"")
        } else {
            String::new()
        };
        let rot = if rotate.abs() > 0.01 {
            format!(" transform=\"rotate({rotate} {x} {y})\"")
        } else {
            String::new()
        };
        // rotate conflicts with motion transform; prefer motion when active
        let open_extra = text_open_attrs(anim, i, width, height);
        let rot_attr = if open_extra.contains("transform=") {
            String::new()
        } else {
            rot
        };
        let children = text_children(anim, i, width, height);
        text_nodes.push_str(&format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" dominant-baseline=\"middle\" \
             font-family=\"ui-sans-serif,system-ui,-apple-system,Segoe UI,Helvetica,sans-serif\" \
             font-weight=\"650\" letter-spacing=\"-0.02em\" font-size=\"{font_size}\" \
             fill=\"{font_color}\"{stroke_attr}{rot_attr}{open_extra}>{content}{children}</text>",
            content = esc(line),
        ));
    }

    let desc_node = if !desc.is_empty() {
        let open_extra = text_open_attrs(anim, lines.len().max(1), width, height);
        let children = text_children(anim, lines.len().max(1), width, height);
        format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"middle\" \
             font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"{desc_size}\" \
             font-weight=\"450\" letter-spacing=\"0.01em\" fill=\"{font_color}\" fill-opacity=\"0.82\"{open_extra}>{}{children}</text>",
            width as f32 * desc_align / 100.0,
            height as f32 * desc_align_y / 100.0,
            esc(desc),
        )
    } else {
        String::new()
    };

    let body = format!(
        "<defs>{}{}</defs>{}{}{}{}",
        fill.defs,
        shape_defs(ty, gain),
        shape_background(ty, width, height, &fill.fill, section, input.reversal, gain),
        text_nodes,
        desc_node,
        credit_mark(width, height, input.credit),
    );

    svg_doc(width, height, &body)
}
