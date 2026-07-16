//! Color / gradient resolution for fills.

use crate::svg::{ensure_hash, is_hex_color, strip_hash};
use crate::themes::{self, Theme};

#[derive(Clone, Debug)]
pub struct FillPlan {
    pub defs: String,
    pub fill: String,
    pub fg: String,
}

pub fn resolve_fill(color: Option<&str>, theme: Option<&str>, seed: &str, gid: &str) -> FillPlan {
    if let Some(name) = theme {
        if let Some(t) = themes::get(name) {
            return theme_fill(t, gid);
        }
    }

    let color = color.unwrap_or("gradient").trim();

    match color {
        "auto" => solid(themes::pick_auto(seed)),
        "timeAuto" => solid(themes::pick_auto(&themes::time_seed())),
        "gradient" | "random" => {
            let (a, b) = themes::pick_gradient(seed);
            gradient(gid, a, b)
        }
        "timeGradient" => {
            let (a, b) = themes::pick_gradient(&themes::time_seed());
            gradient(gid, a, b)
        }
        other => {
            if let Some(plan) = parse_custom_gradient(other, gid) {
                return plan;
            }
            let h = strip_hash(other);
            if is_hex_color(h) {
                solid(h)
            } else {
                let (a, b) = themes::pick_gradient(seed);
                gradient(gid, a, b)
            }
        }
    }
}

fn theme_fill(t: &Theme, gid: &str) -> FillPlan {
    FillPlan {
        defs: gradient_def(gid, t.bg, t.bg2),
        fill: format!("url(#{gid})"),
        fg: t.fg.to_string(),
    }
}

fn solid(hex: &str) -> FillPlan {
    let h = strip_hash(hex);
    FillPlan {
        defs: String::new(),
        fill: ensure_hash(h),
        fg: contrasting_fg(h),
    }
}

fn gradient(gid: &str, a: &str, b: &str) -> FillPlan {
    FillPlan {
        defs: gradient_def(gid, a, b),
        fill: format!("url(#{gid})"),
        fg: "FFFFFF".into(),
    }
}

fn gradient_def(id: &str, a: &str, b: &str) -> String {
    format!(
        r##"<linearGradient id="{id}" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="{}"/><stop offset="100%" stop-color="{}"/></linearGradient>"##,
        ensure_hash(a),
        ensure_hash(b)
    )
}

/// `0:EEFF00,100:a82da8`
fn parse_custom_gradient(color: &str, gid: &str) -> Option<FillPlan> {
    if !color.contains(':') {
        return None;
    }
    let mut stops = Vec::new();
    for part in color.split(',') {
        let mut it = part.trim().splitn(2, ':');
        let off = it.next()?.parse::<u32>().ok()?;
        let col = it.next()?;
        if !is_hex_color(col) {
            return None;
        }
        stops.push((off, ensure_hash(strip_hash(col))));
    }
    if stops.len() < 2 {
        return None;
    }
    let mut xml = format!(r##"<linearGradient id="{gid}" x1="0%" y1="0%" x2="100%" y2="100%">"##);
    for (off, col) in stops {
        xml.push_str(&format!(r##"<stop offset="{off}%" stop-color="{col}"/>"##));
    }
    xml.push_str("</linearGradient>");
    Some(FillPlan {
        defs: xml,
        fill: format!("url(#{gid})"),
        fg: "FFFFFF".into(),
    })
}

pub fn contrasting_fg(hex: &str) -> String {
    let h = strip_hash(hex);
    let full = if h.len() == 3 {
        h.chars().flat_map(|c| [c, c]).collect::<String>()
    } else {
        h.chars().take(6).collect()
    };
    let r = u8::from_str_radix(&full[0..2], 16).unwrap_or(0) as f32;
    let g = u8::from_str_radix(&full[2..4], 16).unwrap_or(0) as f32;
    let b = u8::from_str_radix(&full[4..6], 16).unwrap_or(0) as f32;
    let lum = (0.299 * r + 0.587 * g + 0.114 * b) / 255.0;
    if lum > 0.55 {
        "1A1A1A".into()
    } else {
        "FFFFFF".into()
    }
}
