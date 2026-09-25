//! Color / gradient resolution for fills.
//!
//! Art kernel rule: every banner owns a **chromatic system**, not a single fill.
//! Theme/base color becomes a multi-stop field + accent/secondary/warm orbs so
//! shapes never fall back to pure white wash or theme-blind hardcodes.

use crate::capabilities::mark::domain::paint::css_color;
use crate::capabilities::mark::domain::svg::{ensure_hash, is_hex_color, strip_hash};
use crate::capabilities::mark::domain::theme::{self, Theme};

/// Resolved paint kit consumed by banner shapes + chrome.
#[derive(Clone, Debug)]
pub(crate) struct FillPlan {
    /// SVG gradient/solid defs to inject under `<defs>`.
    pub defs: String,
    /// Main field paint (`url(#…)` or `#hex`).
    pub fill: String,
    /// Primary text/ink (no leading `#` for historical callers).
    pub fg: String,
    /// Deep base (dark end of field).
    pub base: String,
    /// Hero accent (orbs, plate tile, rules).
    pub accent: String,
    /// Cool secondary (meshes, secondary blobs).
    pub accent2: String,
    /// Warm highlight (sparks, tertiary blobs).
    pub warm: String,
    /// Soft specular tint (never pure white).
    pub glow: String,
}

impl FillPlan {
    /// `#rrggbb` form of ink for SVG fill attributes.
    pub(crate) fn fg_hash(&self) -> String {
        ensure_hash(&self.fg)
    }
}

/// Resolve the chromatic paint kit for a mark. Pure and deterministic:
/// the same inputs always produce the same kit — the clock is never sampled
/// (ADR-0003: every mark is a pure function of its URL).
pub(crate) fn resolve_fill(
    color: Option<&str>,
    theme: Option<&str>,
    seed: &str,
    gid: &str,
) -> FillPlan {
    if let Some(name) = theme {
        if let Some(t) = theme::get(name) {
            return theme_fill(t, gid);
        }
    }

    let color = color.unwrap_or("gradient").trim();

    match color {
        "auto" => solid_kit(gid, theme::pick_auto(seed)),
        "gradient" | "random" => {
            let (a, b) = theme::pick_gradient(seed);
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
                let (a, b) = theme::pick_gradient(seed);
                gradient_kit(gid, a, b)
            }
        }
    }
}

fn theme_fill(t: &Theme, gid: &str) -> FillPlan {
    kit(gid, Chroma::themed(t))
}

fn solid_kit(gid: &str, hex: &str) -> FillPlan {
    kit(gid, Chroma::solid(hex))
}

fn gradient_kit(gid: &str, a: &str, b: &str) -> FillPlan {
    kit(gid, Chroma::pair(a, b))
}

/// One field's chromatic roles.
///
/// Each colour source builds the roles in one place, so the SVG gradient stops
/// and the resolved `FillPlan` cannot drift apart through positional arguments
/// (and no call site repeats the literal).
struct Chroma {
    base: String,
    mid: String,
    end: String,
    edge: String,
    warm: String,
    glow: String,
    fg: String,
}

impl Chroma {
    /// Theme pack: mix the pack's own tones.
    fn themed(t: &Theme) -> Self {
        Self {
            base: ensure_hash(t.bg),
            mid: ensure_hash(&mix_hex(t.bg, t.bg2, 0.42)),
            end: ensure_hash(t.bg2),
            edge: ensure_hash(t.accent),
            // Keep warm chromatic — mix accent toward amber, not white.
            warm: ensure_hash(&mix_hex(t.accent, "FEE140", 0.42)),
            glow: ensure_hash(&mix_hex(t.bg2, "FFFFFF", 0.42)),
            fg: strip_hash(&ensure_hash(t.fg)).to_string(),
        }
    }

    /// Single hex base: derive the field and the supporting roles.
    fn solid(hex: &str) -> Self {
        let h = strip_hash(hex);
        Self {
            base: ensure_hash(&darken(h, 0.42)),
            mid: ensure_hash(h),
            end: ensure_hash(&mix_hex(h, "4FACFE", 0.48)),
            edge: ensure_hash(&lighten(h, 0.22)),
            warm: ensure_hash(&mix_hex(h, "FEE140", 0.5)),
            glow: ensure_hash(&mix_hex(h, "FFFFFF", 0.48)),
            fg: contrasting_fg(h),
        }
    }

    /// Two gradient endpoints: darken A for depth, keep B chroma high.
    fn pair(a: &str, b: &str) -> Self {
        Self {
            base: ensure_hash(&darken(a, 0.22)),
            mid: ensure_hash(&mix_hex(a, b, 0.45)),
            end: ensure_hash(b),
            edge: ensure_hash(&lighten(b, 0.08)),
            warm: ensure_hash(&mix_hex(b, "FEE140", 0.38)),
            glow: ensure_hash(&mix_hex(b, "FFFFFF", 0.4)),
            fg: "FFFFFF".into(),
        }
    }

    /// Exact user stops: first stop, chosen midpoint, last stop.
    fn stops(a: &str, mid: String, b: &str) -> Self {
        Self {
            base: ensure_hash(&darken(strip_hash(a), 0.18)),
            mid,
            end: b.to_string(),
            edge: ensure_hash(&lighten(strip_hash(b), 0.06)),
            warm: ensure_hash(&mix_hex(strip_hash(b), "FEE140", 0.35)),
            glow: ensure_hash(&mix_hex(strip_hash(b), "FFFFFF", 0.4)),
            fg: "FFFFFF".into(),
        }
    }
}

fn kit(gid: &str, c: Chroma) -> FillPlan {
    FillPlan {
        defs: chromatic_defs(gid, &c),
        fill: format!("url(#{gid})"),
        fg: strip_hash(&c.fg).to_string(),
        base: c.base.clone(),
        accent: c.edge.clone(),
        accent2: c.end.clone(),
        warm: c.warm.clone(),
        glow: c.glow.clone(),
    }
}

/// Field + chroma utilities referenced by shapes/motion.
fn chromatic_defs(id: &str, c: &Chroma) -> String {
    let (base, mid, end, edge, warm, glow) = (
        c.base.as_str(),
        c.mid.as_str(),
        c.end.as_str(),
        c.edge.as_str(),
        c.warm.as_str(),
        c.glow.as_str(),
    );
    format!(
        r##"<linearGradient id="{id}" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="{base}"/>
          <stop offset="34%" stop-color="{mid}"/>
          <stop offset="68%" stop-color="{end}"/>
          <stop offset="100%" stop-color="{edge}"/>
        </linearGradient>
        <radialGradient id="{id}Bloom" cx="74%" cy="16%" r="72%">
          <stop offset="0%" stop-color="{edge}" stop-opacity="0.55"/>
          <stop offset="42%" stop-color="{end}" stop-opacity="0.22"/>
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>
        </radialGradient>
        <radialGradient id="{id}Bloom2" cx="18%" cy="78%" r="65%">
          <stop offset="0%" stop-color="{warm}" stop-opacity="0.34"/>
          <stop offset="55%" stop-color="{end}" stop-opacity="0.1"/>
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>
        </radialGradient>
        <linearGradient id="{id}Sheen" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="{glow}" stop-opacity="0.2"/>
          <stop offset="42%" stop-color="{edge}" stop-opacity="0.04"/>
          <stop offset="100%" stop-color="{base}" stop-opacity="0"/>
        </linearGradient>
        <radialGradient id="{id}Vig" cx="50%" cy="38%" r="78%">
          <stop offset="0%" stop-color="{glow}" stop-opacity="0"/>
          <stop offset="70%" stop-color="{base}" stop-opacity="0.08"/>
          <stop offset="100%" stop-color="{base}" stop-opacity="0.38"/>
        </radialGradient>
        <linearGradient id="{id}Holo" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="{edge}" stop-opacity="0"/>
          <stop offset="28%" stop-color="{end}" stop-opacity="0.42"/>
          <stop offset="52%" stop-color="{warm}" stop-opacity="0.34"/>
          <stop offset="74%" stop-color="{edge}" stop-opacity="0.28"/>
          <stop offset="100%" stop-color="{end}" stop-opacity="0"/>
        </linearGradient>
        <linearGradient id="{id}Drift" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stop-color="{edge}" stop-opacity="0">
          </stop>
          <stop offset="45%" stop-color="{warm}" stop-opacity="0.22">
          </stop>
          <stop offset="100%" stop-color="{end}" stop-opacity="0">
          </stop>
        </linearGradient>
        <linearGradient id="{id}WaveA" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="{edge}" stop-opacity="0.55"/>
          <stop offset="100%" stop-color="{end}" stop-opacity="0.22"/>
        </linearGradient>
        <linearGradient id="{id}WaveB" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="{warm}" stop-opacity="0.5"/>
          <stop offset="100%" stop-color="{end}" stop-opacity="0.18"/>
        </linearGradient>
        <linearGradient id="{id}WaveC" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="{glow}" stop-opacity="0.42"/>
          <stop offset="100%" stop-color="{mid}" stop-opacity="0.14"/>
        </linearGradient>"##
    )
}

fn parse_custom_gradient(spec: &str, gid: &str) -> Option<FillPlan> {
    // Formats: "0:EEFF00,100:a82da8" or "FF6B6B,C44569,F8B500"
    let parts: Vec<&str> = spec
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() {
        return None;
    }

    let mut stops: Vec<(f32, String)> = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if let Some((off, hex)) = p.split_once(':') {
            let o: f32 = off.parse().ok()?;
            // SVG offsets must be finite percentages in the public grammar.
            // Reject malformed values instead of serializing `NaN%`/`inf%`.
            if !o.is_finite() || !(0.0..=100.0).contains(&o) {
                return None;
            }
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

    let mut plan = kit(gid, Chroma::stops(&a, mid, &b));

    // Rebuild primary field gradient with exact user stop positions.
    let mut stop_svg = String::new();
    for (o, c) in &stops {
        stop_svg.push_str(&format!("<stop offset=\"{o}%\" stop-color=\"{c}\"/>"));
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

/// Deep neutral canvas for restrained art (ADR-0004): dark bases deepen to a
/// near-black ink with a hue tint (capsule-class negative space); light bases
/// stay as-is so light themes keep a light canvas.
pub(crate) fn ink_canvas(base: &str) -> String {
    let h = strip_hash(base);
    if contrasting_fg(h) == "FFFFFF" {
        ensure_hash(&mix_hex(h, "0B0E14", 0.7))
    } else {
        ensure_hash(h)
    }
}

pub(crate) fn contrasting_fg(hex: &str) -> String {
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

/// Resolve one paint token: any color spelling shields accepts (named, hex,
/// CSS name, `rgb()`/`hsl()`), else the caller's fallback. Anything else is
/// dropped instead of reaching an SVG attribute — the paint grammar has
/// exactly one entry point.
pub(crate) fn resolve_paint(c: Option<&str>, fallback: &str) -> String {
    c.and_then(css_color)
        .unwrap_or_else(|| fallback.to_string())
}

/// WCAG 2.x relative luminance of a six-digit hex color (`#` optional).
/// Anything that is not six hex digits reads as black.
pub(crate) fn relative_luminance(hex: &str) -> f64 {
    let h = strip_hash(hex);
    let channel = |i: usize| -> f64 {
        let v = h
            .get(i..i + 2)
            .and_then(|s| u8::from_str_radix(s, 16).ok())
            .unwrap_or(0) as f64
            / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    if h.len() != 6 {
        return 0.0;
    }
    0.2126 * channel(0) + 0.7152 * channel(2) + 0.0722 * channel(4)
}

/// WCAG 2.x contrast ratio between two hex colors (1.0 ..= 21.0).
pub(crate) fn contrast_ratio(a: &str, b: &str) -> f64 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// Title ink that stays legible over a bright art wash.
///
/// A theme's foreground is designed for its base, but full-bleed art paints
/// the field toward the accent. The field is estimated as base mixed halfway
/// to accent; when the theme ink falls under 3:1 against it, the ink moves to
/// the same side (light ink → near-white, dark ink → near-black), never across,
/// so text on a deep ink canvas cannot flip to dark.
pub(crate) fn legible_ink(fg: &str, base: &str, accent: &str) -> String {
    let field = mix_hex(strip_hash(base), strip_hash(accent), 0.5);
    if contrast_ratio(fg, &field) >= 3.0 {
        return strip_hash(fg).to_string();
    }
    let side = if relative_luminance(fg) >= 0.18 {
        "F8FAFC"
    } else {
        "0F172A"
    };
    if contrast_ratio(side, &field) > contrast_ratio(fg, &field) {
        side.to_string()
    } else {
        strip_hash(fg).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legible_ink_lifts_muted_theme_ink_over_bright_art() {
        // tokyonight: lavender ink over a base→blue wash reads ~1.5:1.
        assert_eq!(legible_ink("A9B1D6", "1A1B27", "7AA2F7"), "F8FAFC");
        // Already legible ink is kept.
        assert_eq!(legible_ink("FFFFFF", "0D1117", "1F6FEB"), "FFFFFF");
        // Dark ink never flips to light.
        assert_ne!(legible_ink("24292F", "FFFFFF", "0969DA"), "F8FAFC");
    }

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
        assert!(
            p.defs.contains("#FF6B6B") || p.defs.contains("#ff6b6b") || p.defs.contains("FF6B6B")
        );
    }

    #[test]
    fn chromatic_defs_do_not_emit_raw_string_continuations() {
        let p = resolve_fill(Some("gradient"), None, "x", "mg");
        assert!(
            !p.defs.contains('\\'),
            "gradient definitions must not contain literal Rust continuation markers"
        );
    }

    #[test]
    fn custom_stops_reject_nonfinite_or_out_of_range_offsets() {
        for spec in [
            "NaN:FF6B6B,100:C44569",
            "inf:FF6B6B,100:C44569",
            "-1:FF6B6B,100:C44569",
            "0:FF6B6B,101:C44569",
        ] {
            let p = resolve_fill(Some(spec), None, "x", "mg");
            assert!(!p.defs.contains("NaN"), "invalid offset escaped: {spec}");
            assert!(!p.defs.contains("inf"), "invalid offset escaped: {spec}");
            assert!(!p.defs.contains("-1%"), "invalid offset escaped: {spec}");
            assert!(!p.defs.contains("101%"), "invalid offset escaped: {spec}");
        }
    }

    #[test]
    fn wcag_contrast_matches_reference_values() {
        assert!((contrast_ratio("#000000", "#FFFFFF") - 21.0).abs() < 1e-9);
        assert!((contrast_ratio("777777", "777777") - 1.0).abs() < 1e-9);
        // GitHub's brand ink on the dark icon tile is unreadable.
        assert!(contrast_ratio("181717", "242938") < 2.5);
        assert!(contrast_ratio("F7DF1E", "242938") > 2.5);
    }
}
