//! Icon tiles: the skill-icons-compatible grid (`GET /icons?i=`).
//!
//! Geometry matches skillicons.dev so a host swap keeps README layouts: each
//! tile is a 256-unit rounded square on a 300-unit step, the viewBox is
//! `cols*300-44` by `rows*300-44`, and the document renders at 48px per tile.

use crate::capabilities::mark::domain::icons::{
    brand_paint, num, paint, resolve, skill_names, Icon, IconArt,
};
use crate::capabilities::mark::domain::svg::{esc, svg_doc_scaled};

pub const DEFAULT_PER_LINE: u32 = 15;
pub const MAX_PER_LINE: u32 = 50;
/// Bounded input: `i=all` is the largest request (the whole skill-icons set).
pub const MAX_TILES: usize = 300;

const TILE: u32 = 256;
const STEP: u32 = 300;
const RENDERED_TILE_PX: f64 = 48.0;
const GLYPH: f64 = 172.0;
const DARK_BG: &str = "#242938";
const LIGHT_BG: &str = "#F4F2ED";

/// Tile theme (skill-icons `theme`/`t`): dark unless `light`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileTheme {
    Dark,
    Light,
}

impl TileTheme {
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
            Some("light") => Self::Light,
            _ => Self::Dark,
        }
    }

    fn bg(self) -> &'static str {
        match self {
            Self::Dark => DARK_BG,
            Self::Light => LIGHT_BG,
        }
    }
}

/// Parse skill-icons `perline`: 1..=50, default 15; bad input is the default.
pub fn parse_per_line(raw: Option<&str>) -> u32 {
    raw.and_then(|s| s.trim().parse::<i64>().ok())
        .map_or(DEFAULT_PER_LINE, |n| n.clamp(1, MAX_PER_LINE as i64) as u32)
}

/// Render the icon grid. Unknown ids are skipped, like skill-icons; a request
/// with no known id renders one lettermark tile so the image never breaks.
pub fn render(ids: &str, theme: TileTheme, per_line: u32) -> String {
    let per = per_line.clamp(1, MAX_PER_LINE);
    let requested: Vec<String> = if ids.trim().eq_ignore_ascii_case("all") {
        skill_names().map(str::to_string).collect()
    } else {
        ids.split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect()
    };
    let mut icons: Vec<(String, Icon)> = requested
        .into_iter()
        .filter_map(|id| resolve(&id).map(|icon| (id, icon)))
        .take(MAX_TILES)
        .collect();
    if icons.is_empty() {
        icons.push((
            "unknown".into(),
            Icon {
                art: IconArt::Mono("?"),
                hex: "8B949E",
            },
        ));
    }

    let n = icons.len() as u32;
    let cols = n.min(per);
    let rows = n.div_ceil(per);
    let view_w = cols * STEP - (STEP - TILE);
    let view_h = rows * STEP - (STEP - TILE);
    let scale = RENDERED_TILE_PX / TILE as f64;
    let bg = theme.bg();
    let offset = num((TILE as f64 - GLYPH) / 2.0);

    let mut body = String::with_capacity(icons.len() * 1200);
    for (i, (id, icon)) in icons.iter().enumerate() {
        let i = i as u32;
        let (x, y) = ((i % per) * STEP, (i / per) * STEP);
        let label = match icon.art {
            IconArt::Brand(b) => b.title,
            _ => id.as_str(),
        };
        body.push_str(&format!(
            "<g transform=\"translate({x},{y})\"><title>{}</title>\
             <rect width=\"{TILE}\" height=\"{TILE}\" rx=\"60\" fill=\"{bg}\"/>\
             <g transform=\"translate({offset},{offset})\">{}</g></g>",
            esc(label),
            paint(icon, GLYPH, &brand_paint(icon.hex, bg))
        ));
    }
    svg_doc_scaled(
        &num(view_w as f64 * scale),
        &num(view_h as f64 * scale),
        view_w,
        view_h,
        &body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_matches_skill_icons() {
        let svg = render("js,ts,rust", TileTheme::Dark, DEFAULT_PER_LINE);
        assert!(svg.contains("width=\"160.5\" height=\"48\" viewBox=\"0 0 856 256\""));
        let svg = render("js,ts,rust,go", TileTheme::Light, 3);
        assert!(svg.contains("viewBox=\"0 0 856 556\""));
        assert!(svg.contains("fill=\"#F4F2ED\""));
        assert!(svg.contains("translate(0,300)"));
    }

    #[test]
    fn low_contrast_brands_switch_ink() {
        let svg = render("github", TileTheme::Dark, 15);
        assert!(svg.contains("fill=\"#FFFFFF\""));
        assert!(!svg.contains("fill=\"#181717\""));
    }

    #[test]
    fn unknown_ids_are_skipped_and_empty_never_breaks() {
        let svg = render("rust,nope-nope,go", TileTheme::Dark, 15);
        assert_eq!(svg.matches("rx=\"60\"").count(), 2);
        let svg = render("nope-nope", TileTheme::Dark, 15);
        assert_eq!(svg.matches("rx=\"60\"").count(), 1);
    }

    #[test]
    fn all_renders_the_whole_skill_set_and_perline_parses() {
        let n = skill_names().count();
        assert_eq!(
            render("all", TileTheme::Dark, 15)
                .matches("rx=\"60\"")
                .count(),
            n
        );
        assert_eq!(parse_per_line(None), 15);
        assert_eq!(parse_per_line(Some("abc")), 15);
        assert_eq!(parse_per_line(Some("0")), 1);
        assert_eq!(parse_per_line(Some("99")), 50);
        assert_eq!(TileTheme::parse(Some("LIGHT")), TileTheme::Light);
        assert_eq!(TileTheme::parse(Some("neon")), TileTheme::Dark);
    }
}
