//! Color / gradient resolution for fills.
//!
//! Art kernel rule: every banner owns a **chromatic system**, not a single fill.
//! Theme/base color becomes a multi-stop field + accent/secondary/warm orbs so
//! shapes never fall back to pure white wash or theme-blind hardcodes.

use crate::svg::{ensure_hash, is_hex_color, strip_hash};
use crate::themes::{self, Theme};

/// Resolved paint kit consumed by banner shapes + chrome.
#[derive(Clone, Debug)]
pub struct FillPlan {
    /// SVG gradient/solid defs to inject under `<defs>`.
    pub defs: String,
    /// Main field paint (`url(#…)` or `#hex`).
    pub fill: String,
    /// Primary text/ink (no leading `#` for historical callers).
    pub fg: String,
    /// Deep base (dark end of field).
    pub base: String,
    /// Mid field tone.
    pub mid: String,
    /// Hero accent (orbs, plate tile, rules).
    pub accent: String,
    /// Cool secondary (meshes, secondary blobs).
    pub accent2: String,
    /// Warm highlight (sparks, tertiary blobs).
    pub warm: String,
    /// Soft specular tint (never pure white).
    pub glow: String,
    /// Muted supporting tone.
    pub muted: String,
}

impl FillPlan {
    /// `#rrggbb` form of ink for SVG fill attributes.
    pub fn fg_hash(&self) -> String {
        ensure_hash(&self.fg)
    }
}

pub fn resolve_fill(color: Option<&str>, theme: Option<&str>, seed: &str, gid: &str) -> FillPlan {
    if let Some(name) = theme {
        if let Some(t) = themes::get(name) {
            return theme_fill(t, gid);
        }
    }

    let color = color.unwrap_or("gradient").trim();

    match color {
        "auto" => solid_kit(gid, themes::pick_auto(seed)),
        "timeAuto" => solid_kit(gid, themes::pick_auto(&themes::time_seed())),
        "gradient" | "random" => {
            let (a, b) = themes::pick_gradient(seed);
            gradient_kit(gid, a, b)
        }
        "timeGradient" => {
            let (a, b) = themes::pick_gradient(&themes::time_seed());
            gradient_kit(gid, a, b)
        }
        other => {
            if let Some(plan) = parse_custom_gradient(other, gid) {
                return plan;
            }
            let h = strip_hash(other);
            if is_hex_color(h) {
                solid_kit(gid, h)
            } else {
                let (a, b) = themes::pick_gradient(seed);
                gradient_kit(gid, a, b)
            }
        }
    }
}

fn theme_fill(t: &Theme, gid: &str) -> FillPlan {
    let base = ensure_hash(t.bg);
    let accent2 = ensure_hash(t.bg2);
    let accent = ensure_hash(t.accent);
    let mid = ensure_hash(&mix_hex(t.bg, t.bg2, 0.42));
    // Keep warm chromatic — mix accent toward amber, not white.
    let warm = ensure_hash(&mix_hex(t.accent, "FEE140", 0.42));
    let glow = ensure_hash(&mix_hex(t.bg2, "FFFFFF", 0.42));
    let muted = ensure_hash(t.muted);
    let fg = ensure_hash(t.fg);

    kit(
        gid,
        &base,
        &mid,
        &accent2,
        &accent,
        &warm,
        &glow,
        &muted,
        strip_hash(&fg),
    )
}

fn solid_kit(gid: &str, hex: &str) -> FillPlan {
    let h = strip_hash(hex);
    let base = ensure_hash(&darken(h, 0.42));
    let mid = ensure_hash(h);
    let accent = ensure_hash(&lighten(h, 0.22));
    let accent2 = ensure_hash(&mix_hex(h, "4FACFE", 0.48));
    let warm = ensure_hash(&mix_hex(h, "FEE140", 0.5));
    let glow = ensure_hash(&mix_hex(h, "FFFFFF", 0.48));
    let muted = ensure_hash(&mix_hex(h, "94A3B8", 0.45));
    let fg = contrasting_fg(h);

    kit(
        gid, &base, &mid, &accent2, &accent, &warm, &glow, &muted, &fg,
    )
}

fn gradient_kit(gid: &str, a: &str, b: &str) -> FillPlan {
    // Prefer saturated endpoints: darken A for depth, keep B chroma high.
    let base = ensure_hash(&darken(a, 0.22));
    let mid = ensure_hash(&mix_hex(a, b, 0.45));
    let accent2 = ensure_hash(b);
    let accent = ensure_hash(&lighten(b, 0.08));
    let warm = ensure_hash(&mix_hex(b, "FEE140", 0.38));
    let glow = ensure_hash(&mix_hex(b, "FFFFFF", 0.4));
    let muted = ensure_hash(&mix_hex(a, "A8B3C7", 0.4));

    kit(
        gid, &base, &mid, &accent2, &accent, &warm, &glow, &muted, "FFFFFF",
    )
}

fn kit(
    gid: &str,
    base: &str,
    mid: &str,
    accent2: &str,
    accent: &str,
    warm: &str,
    glow: &str,
    muted: &str,
    fg: &str,
) -> FillPlan {
    FillPlan {
        defs: chromatic_defs(gid, base, mid, accent2, accent, warm, glow),
        fill: format!("url(#{gid})"),
        fg: strip_hash(fg).to_string(),
        base: base.to_string(),
        mid: mid.to_string(),
        accent: accent.to_string(),
        accent2: accent2.to_string(),
        warm: warm.to_string(),
        glow: glow.to_string(),
        muted: muted.to_string(),
    }
}

/// Field + chroma utilities referenced by shapes/motion.
fn chromatic_defs(
    id: &str,
    base: &str,
    mid: &str,
    end: &str,
    edge: &str,
    warm: &str,
    glow: &str,
) -> String {
    format!(
        r##"<linearGradient id="{id}" x1="0%" y1="0%" x2="100%" y2="100%">\
          <stop offset="0%" stop-color="{base}"/>\
          <stop offset="34%" stop-color="{mid}"/>\
          <stop offset="68%" stop-color="{end}"/>\
          <stop offset="100%" stop-color="{edge}"/>\
        </linearGradient>\
        <radialGradient id="{id}Bloom" cx="74%" cy="16%" r="72%">\
          <stop offset="0%" stop-color="{edge}" stop-opacity="0.55"/>\
          <stop offset="42%" stop-color="{end}" stop-opacity="0.22"/>\
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>\
        </radialGradient>\
        <radialGradient id="{id}Bloom2" cx="18%" cy="78%" r="65%">\
          <stop offset="0%" stop-color="{warm}" stop-opacity="0.34"/>\
          <stop offset="55%" stop-color="{end}" stop-opacity="0.1"/>\
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>\
        </radialGradient>\
        <linearGradient id="{id}Sheen" x1="0%" y1="0%" x2="0%" y2="100%">\
          <stop offset="0%" stop-color="{glow}" stop-opacity="0.2"/>\
          <stop offset="42%" stop-color="{edge}" stop-opacity="0.04"/>\
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>\
        </linearGradient>\
        <radialGradient id="{id}Vig" cx="50%" cy="38%" r="78%">\
          <stop offset="0%" stop-color="{glow}" stop-opacity="0"/>\
          <stop offset="70%" stop-color="{base}" stop-opacity="0.08"/>\
          <stop offset="100%" stop-color="{base}" stop-opacity="0.38"/>\
        </radialGradient>\
        <linearGradient id="{id}Holo" x1="0%" y1="0%" x2="100%" y2="100%">\
          <stop offset="0%" stop-color="{edge}" stop-opacity="0"/>\
          <stop offset="28%" stop-color="{end}" stop-opacity="0.42"/>\
          <stop offset="52%" stop-color="{warm}" stop-opacity="0.34"/>\
          <stop offset="74%" stop-color="{edge}" stop-opacity="0.28"/>\
          <stop offset="100%" stop-color="{end}" stop-opacity="0"/>\
        </linearGradient>\
        <linearGradient id="{id}Drift" x1="0%" y1="0%" x2="100%" y2="0%">\
          <stop offset="0%" stop-color="{edge}" stop-opacity="0">\\
          </stop>\
          <stop offset="45%" stop-color="{warm}" stop-opacity="0.22">\\
          </stop>\
          <stop offset="100%" stop-color="{end}" stop-opacity="0">\\
          </stop>\
        </linearGradient>\
        <linearGradient id="{id}WaveA" x1="0%" y1="0%" x2="0%" y2="100%">\
          <stop offset="0%" stop-color="{edge}" stop-opacity="0.55"/>\
          <stop offset="100%" stop-color="{end}" stop-opacity="0.22"/>\
        </linearGradient>\
        <linearGradient id="{id}WaveB" x1="0%" y1="0%" x2="0%" y2="100%">\
          <stop offset="0%" stop-color="{warm}" stop-opacity="0.5"/>\
          <stop offset="100%" stop-color="{end}" stop-opacity="0.18"/>\
        </linearGradient>\
        <linearGradient id="{id}WaveC" x1="0%" y1="0%" x2="0%" y2="100%">\
          <stop offset="0%" stop-color="{glow}" stop-opacity="0.42"/>\
          <stop offset="100%" stop-color="{mid}" stop-opacity="0.14"/>\
        </linearGradient>"##
    )
}

fn parse_custom_gradient(spec: &str, gid: &str) -> Option<FillPlan> {
    // Formats: "0:EEFF00,100:a82da8" or "FF6B6B,C44569,F8B500"
    let parts: Vec<&str> = spec.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        return None;
    }

    let mut stops: Vec<(f32, String)> = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if let Some((off, hex)) = p.split_once(':') {
            let o: f32 = off.parse().ok()?;
            let h = strip_hash(hex);
            if !is_hex_color(h) {
                return None;
            }
            stops.push((o, ensure_hash(h)));
        } else {
            let h = strip_hash(p);
            if !is_hex_color(h) {
                return None;
            }
            let o = if parts.len() == 1 {
                0.0
            } else {
                (i as f32) * 100.0 / (parts.len() as f32 - 1.0)
            };
            stops.push((o, ensure_hash(h)));
        }
    }
    stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let a = stops.first()?.1.clone();
    let b = stops.last()?.1.clone();
    let mid = if stops.len() >= 3 {
        stops[stops.len() / 2].1.clone()
    } else {
        ensure_hash(&mix_hex(strip_hash(&a), strip_hash(&b), 0.5))
    };

    let base = ensure_hash(&darken(strip_hash(&a), 0.18));
    let accent = ensure_hash(&lighten(strip_hash(&b), 0.06));
    let accent2 = b.clone();
    let warm = ensure_hash(&mix_hex(strip_hash(&b), "FEE140", 0.35));
    let glow = ensure_hash(&mix_hex(strip_hash(&b), "FFFFFF", 0.4));
    let muted = ensure_hash(&mix_hex(strip_hash(&a), "A8B3C7", 0.4));

    let mut plan = kit(
        gid,
        &base,
        &mid,
        &accent2,
        &accent,
        &warm,
        &glow,
        &muted,
        "FFFFFF",
    );

    // Rebuild primary field gradient with exact user stop positions.
    let mut stop_svg = String::new();
    for (o, c) in &stops {
        stop_svg.push_str(&format!(
            "<stop offset=\"{o}%\" stop-color=\"{c}\"/>"
        ));
    }
    let field = format!(
        "<linearGradient id=\"{gid}\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">{stop_svg}</linearGradient>"
    );
    let marker = format!("id=\"{gid}Bloom\"");
    if let Some(pos) = plan.defs.find(&marker) {
        if let Some(tag) = plan.defs[..pos].rfind("<radialGradient") {
            plan.defs = field + &plan.defs[tag..];
        }
    }
    Some(plan)
}

pub fn contrasting_fg(hex: &str) -> String {
    let h = strip_hash(hex);
    if h.len() != 6 {
        return "FFFFFF".into();
    }
    let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0) as f32;
    let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0) as f32;
    let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0) as f32;
    // Relative luminance
    let l = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;
    if l > 0.55 {
        "0F172A".into()
    } else {
        "FFFFFF".into()
    }
}

fn darken(hex: &str, amount: f32) -> String {
    mix_hex(hex, "000000", amount.clamp(0.0, 1.0))
}

fn lighten(hex: &str, amount: f32) -> String {
    mix_hex(hex, "FFFFFF", amount.clamp(0.0, 1.0))
}

fn mix_hex(a: &str, b: &str, t: f32) -> String {
    let a = strip_hash(a);
    let b = strip_hash(b);
    let parse = |h: &str, i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f32;
    if a.len() != 6 || b.len() != 6 {
        return a.to_string();
    }
    let t = t.clamp(0.0, 1.0);
    let mix = |i: usize| {
        let v = parse(a, i) * (1.0 - t) + parse(b, i) * t;
        format!("{:02X}", v.round().clamp(0.0, 255.0) as u8)
    };
    format!("{}{}{}", mix(0), mix(2), mix(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_plan_has_chroma_roles() {
        let p = resolve_fill(None, Some("sunset"), "seed", "mg");
        assert!(p.fill.contains("url(#mg)"));
        assert!(p.defs.contains("id=\"mg\""));
        assert!(p.defs.contains("mgBloom"));
        assert!(p.defs.contains("mgHolo"));
        assert!(p.defs.contains("mgWaveA"));
        assert_ne!(p.base.to_ascii_lowercase(), p.accent.to_ascii_lowercase());
        assert_ne!(p.accent.to_ascii_lowercase(), p.warm.to_ascii_lowercase());
    }

    #[test]
    fn gradient_default_is_chromatic() {
        let p = resolve_fill(Some("gradient"), None, "wave-Ship", "mg");
        assert!(p.fill.starts_with("url(#"));
        assert!(p.defs.contains("linearGradient"));
        // Must not be a pure solid white/black kit.
        assert!(!p.accent.eq_ignore_ascii_case("#ffffff"));
        assert!(!p.accent2.eq_ignore_ascii_case("#000000"));
    }

    #[test]
    fn custom_stops_parse() {
        let p = resolve_fill(Some("0:FF6B6B,100:C44569"), None, "x", "mg");
        assert!(p.defs.contains("#FF6B6B") || p.defs.contains("#ff6b6b") || p.defs.contains("FF6B6B"));
    }
}
