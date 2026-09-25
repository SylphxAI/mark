//! Card palettes: github-readme-stats theme names plus Mark's neutral themes.
//!
//! A drop-in URL (`?theme=radical`) must look like what the user chose, so the
//! github-readme-stats names map to their published colors. Names it does not
//! know fall back to Mark's neutral theme catalog, then to the default.

use crate::capabilities::mark::domain::svg::normalize_hex_token;
use crate::capabilities::mark::domain::theme;

/// Card background: a solid color or a linear gradient (`angle,c1,c2,…`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Background {
    Solid(String),
    Gradient { angle: f32, stops: Vec<String> },
}

/// Resolved card colors, every value a canonical `#rrggbb[aa]` token.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Palette {
    pub title: String,
    pub icon: String,
    pub text: String,
    pub bg: Background,
    pub border: String,
}

/// (name, title, icon, text, bg, border). Border `""` means the default
/// hairline (`e4e2e2`), as github-readme-stats does.
type Row = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

static THEMES: &[Row] = &[
    ("default", "2f80ed", "4c71f2", "434d58", "fffefe", ""),
    (
        "default_repocard",
        "2f80ed",
        "586069",
        "434d58",
        "fffefe",
        "",
    ),
    ("transparent", "006aff", "0579c3", "417e87", "ffffff00", ""),
    ("dark", "ffffff", "79ff97", "9f9f9f", "151515", ""),
    ("radical", "fe428e", "f8d847", "a9fef7", "141321", ""),
    ("merko", "abd200", "b7d364", "68b587", "0a0f0b", ""),
    ("gruvbox", "fabd2f", "fe8019", "8ec07c", "282828", ""),
    ("gruvbox_light", "b57614", "af3a03", "427b58", "fbf1c7", ""),
    ("tokyonight", "70a5fd", "bf91f3", "38bdae", "1a1b27", ""),
    ("onedark", "e4bf7a", "8eb573", "df6d74", "282c34", ""),
    ("cobalt", "e683d9", "0480ef", "75eeb2", "193549", ""),
    ("synthwave", "e2e9ec", "ef8539", "e5289e", "2b213a", ""),
    ("highcontrast", "e7f216", "00ffff", "ffffff", "000000", ""),
    ("dracula", "ff6e96", "79dafa", "f8f8f2", "282a36", ""),
    ("prussian", "bddfff", "38a0ff", "6e93b5", "172f45", ""),
    ("monokai", "eb1f6a", "e28905", "f1f1eb", "272822", ""),
    ("vue", "41b883", "41b883", "273849", "fffefe", ""),
    ("vue-dark", "41b883", "41b883", "fffefe", "273849", ""),
    (
        "shades-of-purple",
        "fad000",
        "b362ff",
        "a599e9",
        "2d2b55",
        "",
    ),
    ("nightowl", "c792ea", "ffeb95", "7fdbca", "011627", ""),
    ("buefy", "7957d5", "ff3860", "363636", "ffffff", ""),
    ("blue-green", "2f97c1", "f5b700", "0cf574", "040f0f", ""),
    ("algolia", "00aeff", "2dde98", "ffffff", "050f2c", ""),
    ("great-gatsby", "ffa726", "ffb74d", "ffd95b", "000000", ""),
    ("darcula", "ba5f17", "84628f", "bebebe", "242424", ""),
    ("bear", "e03c8a", "00aeff", "bcb28d", "1f2023", ""),
    ("solarized-dark", "268bd2", "b58900", "859900", "002b36", ""),
    (
        "solarized-light",
        "268bd2",
        "b58900",
        "859900",
        "fdf6e3",
        "",
    ),
    (
        "chartreuse-dark",
        "7fff00",
        "00aeff",
        "ffffff",
        "000000",
        "",
    ),
    ("nord", "81a1c1", "88c0d0", "d8dee9", "2e3440", ""),
    ("gotham", "2aa889", "599cab", "99d1ce", "0c1014", ""),
    (
        "material-palenight",
        "c792ea",
        "89ddff",
        "a6accd",
        "292d3e",
        "",
    ),
    ("graywhite", "24292e", "24292e", "24292e", "ffffff", ""),
    (
        "vision-friendly-dark",
        "ffb000",
        "785ef0",
        "ffffff",
        "000000",
        "",
    ),
    ("ayu-mirage", "f4cd7c", "73d0ff", "c7c8c2", "1f2430", ""),
    (
        "midnight-purple",
        "9745f5",
        "9f4bff",
        "ffffff",
        "000000",
        "",
    ),
    ("calm", "e07a5f", "edae49", "ebcfb2", "373f51", ""),
    ("omni", "ff79c6", "e7de79", "e1e1e6", "191622", ""),
    ("react", "61dafb", "61dafb", "ffffff", "20232a", ""),
    ("jolly", "ff64da", "a960ff", "ffffff", "291b3e", ""),
    ("maroongold", "f7ef8a", "f7ef8a", "e0aa3e", "260000", ""),
    ("yeblu", "ffff00", "ffff00", "ffffff", "002046", ""),
    ("blueberry", "82aaff", "89ddff", "27e8a7", "242938", ""),
    ("slateorange", "faa627", "faa627", "ffffff", "36393f", ""),
    ("kacho_ga", "bf4a3f", "a64833", "d9c8a9", "402b23", ""),
    ("outrun", "ffcc00", "ff1aff", "8080ff", "141439", ""),
    ("ocean_dark", "8957b2", "ffffff", "92d534", "151a28", ""),
    ("city_lights", "5d8cb3", "4798ff", "718ca1", "1d252c", ""),
    (
        "github_dark",
        "58a6ff",
        "1f6feb",
        "c3d1d9",
        "0d1117",
        "30363d",
    ),
    (
        "github_dark_dimmed",
        "539bf5",
        "539bf5",
        "adbac7",
        "24292f",
        "373e47",
    ),
    (
        "discord_old_blurple",
        "7289da",
        "7289da",
        "ffffff",
        "2c2f33",
        "",
    ),
    ("aura_dark", "ff7372", "6cffd0", "dbdbdb", "252334", ""),
    ("panda", "19f9d8", "19f9d8", "ff75b5", "31353a", ""),
    ("noctis_minimus", "d3b692", "72b7c0", "c5cdd3", "1b2932", ""),
    ("cobalt2", "ffc600", "ffffff", "0088ff", "193549", ""),
    ("swift", "000000", "f05237", "000000", "f7f7f7", ""),
    ("aura", "a277ff", "ffca85", "61ffca", "15141b", ""),
    ("apprentice", "ffffff", "ffffaf", "bcbcbc", "262626", ""),
    ("moltack", "86092c", "86092c", "574038", "f5e1c0", ""),
    (
        "codestackr",
        "ff652f",
        "ffe400",
        "ffffff",
        "09131b",
        "0c1a25",
    ),
    ("rose_pine", "9ccfd8", "ebbcba", "e0def4", "191724", ""),
    (
        "catppuccin_latte",
        "137980",
        "8839ef",
        "4c4f69",
        "eff1f5",
        "",
    ),
    (
        "catppuccin_mocha",
        "94e2d5",
        "cba6f7",
        "cdd6f4",
        "1e1e2e",
        "",
    ),
    (
        "date_night",
        "da7885",
        "bb8470",
        "e1b2a2",
        "170f0c",
        "170f0c",
    ),
    (
        "one_dark_pro",
        "61afef",
        "c678dd",
        "e5c06e",
        "23272e",
        "3b4048",
    ),
    ("rose", "8d192b", "b71f36", "862931", "e9d8d4", "e9d8d4"),
    ("holi", "5fabee", "5fabee", "d6e7ff", "030314", "85a4c0"),
    ("neon", "00eaff", "00eaff", "ff10f0", "000000", "ffffff"),
    (
        "ambient_gradient",
        "ffffff",
        "ffffff",
        "ffffff",
        "35,4158d0,c850c0,ffcc70",
        "",
    ),
];

const DEFAULT_BORDER: &str = "#e4e2e2";

fn hex(v: &str) -> Option<String> {
    normalize_hex_token(v).map(|h| h.to_ascii_lowercase())
}

/// `bg_color`: a hex token, or `angle,c1,c2[,…]` for a linear gradient.
pub(crate) fn parse_background(raw: &str) -> Option<Background> {
    let parts: Vec<&str> = raw.split(',').map(str::trim).collect();
    if parts.len() >= 3 {
        let angle: f32 = parts[0].parse().ok().filter(|a: &f32| a.is_finite())?;
        let stops: Option<Vec<String>> = parts[1..].iter().take(8).map(|c| hex(c)).collect();
        return Some(Background::Gradient {
            angle: angle.rem_euclid(360.0),
            stops: stops?,
        });
    }
    hex(raw).map(Background::Solid)
}

fn from_row(row: &Row) -> Palette {
    let (_, title, icon, text, bg, border) = *row;
    let pick = |v: &str| hex(v).unwrap_or_else(|| "#000000".into());
    Palette {
        title: pick(title),
        icon: pick(icon),
        text: pick(text),
        bg: parse_background(bg).unwrap_or(Background::Solid("#fffefe".into())),
        border: hex(border).unwrap_or_else(|| DEFAULT_BORDER.into()),
    }
}

/// Palette for a theme name: github-readme-stats names first (case- and
/// separator-insensitive), then Mark's neutral themes, then `default`.
pub(crate) fn palette(name: Option<&str>) -> Palette {
    let norm = |s: &str| s.to_ascii_lowercase().replace('-', "_");
    if let Some(name) = name.map(norm) {
        if let Some(row) = THEMES.iter().find(|r| norm(r.0) == name) {
            return from_row(row);
        }
        if let Some(t) = theme::get(&name) {
            let pick = |v: &str| hex(v).unwrap_or_else(|| "#000000".into());
            return Palette {
                title: pick(t.accent),
                icon: pick(t.accent),
                text: pick(t.fg),
                bg: Background::Solid(pick(t.bg)),
                border: pick(t.bg2),
            };
        }
    }
    from_row(&THEMES[0])
}

/// The explicit color knobs every card accepts, applied over the theme.
#[derive(Debug, Clone, Default)]
pub(crate) struct ColorOverrides {
    pub title: Option<String>,
    pub icon: Option<String>,
    pub text: Option<String>,
    pub bg: Option<String>,
    pub border: Option<String>,
}

impl Palette {
    pub(crate) fn with(mut self, o: &ColorOverrides) -> Self {
        let set = |slot: &mut String, v: &Option<String>| {
            if let Some(h) = v.as_deref().and_then(hex) {
                *slot = h;
            }
        };
        set(&mut self.title, &o.title);
        set(&mut self.icon, &o.icon);
        set(&mut self.text, &o.text);
        set(&mut self.border, &o.border);
        if let Some(bg) = o.bg.as_deref().and_then(parse_background) {
            self.bg = bg;
        }
        self
    }

    /// A representative solid color for the background (contrast decisions).
    pub(crate) fn bg_solid(&self) -> &str {
        match &self.bg {
            Background::Solid(c) => c,
            Background::Gradient { stops, .. } => {
                stops.first().map(String::as_str).unwrap_or("#ffffff")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_stats_names_resolve_to_their_colors() {
        let p = palette(Some("radical"));
        assert_eq!(p.title, "#fe428e");
        assert_eq!(p.bg, Background::Solid("#141321".into()));
        assert_eq!(
            palette(Some("vue_dark")).bg,
            Background::Solid("#273849".into())
        );
        assert_eq!(palette(Some("nope")), palette(None));
        // Mark-only neutral theme.
        assert_ne!(palette(Some("ocean")), palette(None));
    }

    #[test]
    fn overrides_validate_and_gradients_parse() {
        let o = ColorOverrides {
            title: Some("ff0000".into()),
            text: Some("\"><script>".into()),
            bg: Some("90,000000,ffffff".into()),
            ..Default::default()
        };
        let p = palette(None).with(&o);
        assert_eq!(p.title, "#ff0000");
        assert_eq!(p.text, "#434d58");
        assert_eq!(
            p.bg,
            Background::Gradient {
                angle: 90.0,
                stops: vec!["#000000".into(), "#ffffff".into()]
            }
        );
        assert!(parse_background("90,zzz,fff").is_none());
    }
}
