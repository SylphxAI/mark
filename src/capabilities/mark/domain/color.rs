//! Palette resolution for art-backed marks (hero, profile).
//!
//! A mark's colour is one designed [`Palette`]: a base, an ink, and three
//! accents. It comes from a theme pack, from `color=` (one colour or gradient
//! stops), or — with neither — the default dark pack. Every value is a
//! canonical `#RRGGBB` token, so user spelling never reaches an attribute.

use crate::capabilities::mark::domain::paint::css_color;
use crate::capabilities::mark::domain::svg::{ensure_hash, is_hex_color, strip_hash};
use crate::capabilities::mark::domain::theme::{self, Theme};

/// The resolved colours of one art-backed mark.
#[derive(Clone, Debug)]
pub(crate) struct Palette {
    /// Base canvas.
    pub bg: String,
    /// Raised surface (window chrome, hairlines).
    pub surface: String,
    /// Text ink, legible on `bg` (≥ 7:1).
    pub ink: String,
    /// Three accents the art paints from.
    pub accents: [String; 3],
    /// Light base with dark ink.
    pub light: bool,
}

impl Palette {
    fn from_theme(t: &Theme) -> Self {
        Self {
            bg: ensure_hash(t.bg),
            surface: ensure_hash(t.bg2),
            ink: ensure_hash(t.fg),
            accents: [
                ensure_hash(t.accent),
                ensure_hash(t.accent2),
                ensure_hash(t.accent3),
            ],
            light: t.is_light(),
        }
    }

    /// A palette around user colours: a very dark first stop becomes the base
    /// (the classic `0:0F172A,…` banner), all-light stops make a light card,
    /// anything else sits on a deep tint of the first accent.
    fn from_colors(stops: &[String]) -> Self {
        let lum = |c: &str| relative_luminance(c);
        let (bg, accents): (String, Vec<String>) = if stops.len() > 1 && lum(&stops[0]) < 0.03 {
            (stops[0].clone(), stops[1..].to_vec())
        } else {
            (String::new(), stops.to_vec())
        };
        let accents: Vec<String> = if accents.len() == 1 {
            let a = &accents[0];
            let mut kin = analogous(a);
            kin.insert(0, a.clone());
            kin
        } else {
            accents
        };
        let pick = |i: usize| accents[i % accents.len()].clone();
        let accents = [pick(0), pick(accents.len().saturating_sub(1)), pick(1)];
        let all_light = accents.iter().all(|a| lum(a) > 0.45);
        let (bg, light) = if !bg.is_empty() {
            (bg, false)
        } else if all_light {
            (mix(&accents[0], "#FFFFFF", 0.9), true)
        } else {
            (mix(&accents[0], "#07080C", 0.9), false)
        };
        let ink = if light { "#0B0D14" } else { "#F4F6FB" }.to_string();
        let surface = mix(&bg, &ink, 0.08);
        Self {
            bg,
            surface,
            ink,
            accents,
            light,
        }
    }
}

/// Resolve the palette for an art-backed mark. Pure: the same inputs always
/// give the same palette (a seeded pick is a hash of the URL content).
pub(crate) fn resolve_palette(color: Option<&str>, theme_id: Option<&str>, seed: &str) -> Palette {
    if let Some(t) = theme_id.and_then(theme::get) {
        return Palette::from_theme(t);
    }
    let default = || Palette::from_theme(theme::get("dark").expect("dark pack"));
    let Some(raw) = color.map(str::trim).filter(|c| !c.is_empty()) else {
        return default();
    };
    match raw.to_ascii_lowercase().as_str() {
        "gradient" | "random" | "auto" | "timegradient" => {
            Palette::from_theme(theme::pick_seeded(seed))
        }
        _ => color_stops(raw)
            .map(|s| Palette::from_colors(&s))
            .unwrap_or_else(default),
    }
}

/// `0:EEFF00,100:a82da8`, `FF6B6B,C44569`, or one colour in any spelling
/// shields accepts; stops come back in offset order. Up to eight stops.
fn color_stops(raw: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() == 1 && !parts[0].contains(':') {
        let c = css_color(parts[0])?;
        return Some(vec![six(&c)?]);
    }
    let mut stops: Vec<(f32, String)> = Vec::new();
    for (i, p) in parts.iter().take(8).enumerate() {
        let (off, hex) = match p.split_once(':') {
            Some((o, h)) => (o.parse::<f32>().ok()?, h),
            None => (i as f32, *p),
        };
        if !off.is_finite() || !(0.0..=100.0).contains(&off) || !is_hex_color(hex) {
            return None;
        }
        stops.push((off, six(&ensure_hash(strip_hash(hex)))?));
    }
    stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    Some(stops.into_iter().map(|s| s.1).collect())
}

/// Canonical `#RRGGBB` from a 3/6/8-digit token (alpha is dropped: art
/// controls its own opacity).
fn six(hex: &str) -> Option<String> {
    let h = strip_hash(hex);
    let full: String = match h.len() {
        3 => h.chars().flat_map(|c| [c, c]).collect(),
        6 | 8 => h[..6].to_string(),
        _ => return None,
    };
    Some(format!("#{}", full.to_ascii_uppercase()))
}

/// Ink that reads on `field`: the brighter-contrast of near-white and near-black.
pub(crate) fn ink_over(field: &str) -> String {
    let (light, dark) = ("#FFFFFF", "#0B0D14");
    if contrast_ratio(light, field) >= contrast_ratio(dark, field) {
        light.into()
    } else {
        dark.into()
    }
}

/// Mix two colours in sRGB: `t = 0` is `a`, `t = 1` is `b`.
pub(crate) fn mix(a: &str, b: &str, t: f32) -> String {
    ensure_hash(&mix_hex(a, b, t))
}

/// Two neighbours of `color` on the colour wheel, for a one-colour palette.
///
/// Neighbours in the yellow–olive band (hue 35°–95°) are skipped: low-opacity
/// light in those hues reads as mud over a dark base, so a lime accent gets
/// green and teal company instead of olive and ochre.
fn analogous(color: &str) -> Vec<String> {
    let base = hue_of(color);
    let muddy = |h: f32| (35.0..=95.0).contains(&h);
    let mut out: Vec<String> = [30.0, -30.0, 60.0, -60.0, 90.0, -90.0]
        .iter()
        .filter(|d| !muddy((base + *d).rem_euclid(360.0)))
        .take(2)
        .map(|d| rotate_hue(color, *d))
        .collect();
    while out.len() < 2 {
        out.push(rotate_hue(color, 120.0 * (out.len() + 1) as f32));
    }
    out
}

/// Hue of a colour in degrees (0 for greys).
pub(crate) fn hue_of(hex: &str) -> f32 {
    let h = strip_hash(hex);
    let c = |i: usize| u8::from_str_radix(h.get(i..i + 2).unwrap_or("00"), 16).unwrap_or(0) as f32;
    let (r, g, b) = (c(0), c(2), c(4));
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let d = max - min;
    if d < 1e-6 {
        return 0.0;
    }
    let sector = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    sector * 60.0
}

/// Rotate a colour's hue by `deg`, keeping lightness and saturation.
pub(crate) fn rotate_hue(hex: &str, deg: f32) -> String {
    let h = strip_hash(hex);
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f32 / 255.0;
    if h.len() != 6 {
        return ensure_hash(h);
    }
    let (r, g, b) = (c(0), c(2), c(4));
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let l = (max + min) / 2.0;
    let d = max - min;
    if d < 1e-6 {
        return ensure_hash(h);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let mut hue = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    hue = (hue + deg).rem_euclid(360.0);
    let c2 = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c2 * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c2 / 2.0;
    let (r1, g1, b1) = match (hue / 60.0) as u32 {
        0 => (c2, x, 0.0),
        1 => (x, c2, 0.0),
        2 => (0.0, c2, x),
        3 => (0.0, x, c2),
        4 => (x, 0.0, c2),
        _ => (c2, 0.0, x),
    };
    let to = |v: f32| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", to(r1), to(g1), to(b1))
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

pub(crate) fn mix_hex(a: &str, b: &str, t: f32) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_colour_is_the_dark_pack() {
        let p = resolve_palette(None, None, "x");
        assert_eq!(p.bg, "#0A0C12");
        assert!(!p.light);
    }

    #[test]
    fn theme_wins_over_colour() {
        let p = resolve_palette(Some("FF0000"), Some("light"), "x");
        assert!(p.light);
        assert_eq!(p.ink, "#0B0D14");
    }

    #[test]
    fn dark_first_stop_becomes_the_base() {
        let p = resolve_palette(Some("0:0F172A,50:5B8CFF,100:FF8A3D"), None, "x");
        assert_eq!(p.bg, "#0F172A");
        assert_eq!(p.accents[0], "#5B8CFF");
        assert_eq!(p.accents[1], "#FF8A3D");
    }

    #[test]
    fn one_colour_builds_a_harmonious_triad() {
        let p = resolve_palette(Some("5B8CFF"), None, "x");
        assert_eq!(p.accents[0], "#5B8CFF");
        assert_ne!(p.accents[1], p.accents[2]);
        assert!(contrast_ratio(&p.ink, &p.bg) >= 7.0);
        // CSS and shields names are colours too.
        assert_eq!(
            resolve_palette(Some("hotpink"), None, "x").accents[0],
            "#FF69B4"
        );
    }

    #[test]
    fn pastel_stops_make_a_light_card() {
        let p = resolve_palette(Some("FBC2EB,A6C1EE"), None, "x");
        assert!(p.light);
        assert!(contrast_ratio(&p.ink, &p.bg) >= 7.0);
    }

    #[test]
    fn malformed_colours_fall_back_to_the_default() {
        for spec in [
            "NaN:FF6B6B,100:C44569",
            "-1:FF6B6B,100:C44569",
            "0:FF6B6B,101:C44569",
            "\" onload=\"x",
            "zzz",
        ] {
            let p = resolve_palette(Some(spec), None, "x");
            assert_eq!(p.bg, "#0A0C12", "{spec}");
        }
    }

    #[test]
    fn seeded_pick_is_stable() {
        let a = resolve_palette(Some("gradient"), None, "seed");
        let b = resolve_palette(Some("gradient"), None, "seed");
        assert_eq!(a.accents, b.accents);
    }

    #[test]
    fn one_colour_palettes_avoid_the_olive_band() {
        let p = resolve_palette(Some("0:0A0D07,100:C3F53C"), None, "x");
        for a in &p.accents[1..] {
            let h = hue_of(a);
            assert!(!(35.0..=95.0).contains(&h), "{a} hue {h}");
        }
        let blue = resolve_palette(Some("5B8CFF"), None, "x");
        assert_eq!(blue.accents.len(), 3);
    }

    #[test]
    fn hue_rotation_round_trips() {
        assert_eq!(rotate_hue("#FF0000", 120.0), "#00FF00");
        assert_eq!(rotate_hue("#808080", 90.0), "#808080");
    }

    #[test]
    fn wcag_contrast_matches_reference_values() {
        assert!((contrast_ratio("#000000", "#FFFFFF") - 21.0).abs() < 1e-9);
        assert!((contrast_ratio("777777", "777777") - 1.0).abs() < 1e-9);
        assert!(contrast_ratio("181717", "242938") < 2.5);
        assert!(contrast_ratio("F7DF1E", "242938") > 2.5);
    }
}
