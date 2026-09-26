//! Theme packs — a small set of designed, neutral palettes.
//!
//! Every theme is one base colour, one ink, and three accents that sit
//! together (the art paints glows, meshes, and waves from the accents). Each
//! was checked against GitHub's light (`#FFFFFF`) and dark (`#0D1117`) README
//! backgrounds: a banner is an opaque card, so what matters is that the card
//! reads as a deliberate object on both and that its ink clears WCAG AA on its
//! own base.

#[derive(Clone, Debug)]
pub(crate) struct Theme {
    /// Base canvas.
    pub bg: &'static str,
    /// Raised surface / hairline tone (cards, borders, window chrome).
    pub bg2: &'static str,
    /// Text ink.
    pub fg: &'static str,
    /// Primary accent.
    pub accent: &'static str,
    /// Secondary accent.
    pub accent2: &'static str,
    /// Tertiary accent.
    pub accent3: &'static str,
}

impl Theme {
    /// A light theme paints dark ink on a light base.
    pub(crate) fn is_light(&self) -> bool {
        super::color::relative_luminance(self.bg) > 0.4
    }
}

/// Resolve a theme id, including retired ids (mapped to the closest pack).
pub(crate) fn get(name: &str) -> Option<&'static Theme> {
    let name = name.trim().to_ascii_lowercase();
    let id = RETIRED
        .iter()
        .find(|r| r.0 == name)
        .map_or(name.as_str(), |r| r.1);
    THEMES.iter().find(|t| t.0 == id).map(|t| &t.1)
}

/// Published ids, in studio order.
pub(crate) fn list_names() -> Vec<&'static str> {
    THEMES.iter().map(|t| t.0).collect()
}

/// Each published theme's colours (`#hex`), for pickers that draw swatches.
pub(crate) fn palettes() -> serde_json::Value {
    let hex = |h: &str| format!("#{h}");
    THEMES
        .iter()
        .map(|(id, t)| {
            (
                id.to_string(),
                serde_json::json!({
                    "bg": hex(t.bg),
                    "fg": hex(t.fg),
                    "accents": [hex(t.accent), hex(t.accent2), hex(t.accent3)],
                }),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into()
}

/// Theme ids published before the curated set, mapped to their closest pack
/// so a README that names one keeps a designed look.
const RETIRED: &[(&str, &str)] = &[
    ("github", "dark"),
    ("tokyonight", "dark"),
    ("radical", "sunset"),
    ("gruvbox", "sunset"),
    ("dracula", "grape"),
    ("neon", "grape"),
    ("nord", "ocean"),
    ("monokai", "forest"),
];

static THEMES: &[(&str, Theme)] = &[
    (
        "dark",
        Theme {
            bg: "0A0C12",
            bg2: "161A24",
            fg: "F4F6FB",
            accent: "5B6CFF",
            accent2: "A35BFF",
            accent3: "22C4EE",
        },
    ),
    (
        "light",
        Theme {
            bg: "FAFAFC",
            bg2: "E7E9F0",
            fg: "0B0D14",
            accent: "5B6CFF",
            accent2: "C27BFF",
            accent3: "38BDF8",
        },
    ),
    (
        "ocean",
        Theme {
            bg: "04111E",
            bg2: "0D2236",
            fg: "EAF6FF",
            accent: "0EA5E9",
            accent2: "2DD4BF",
            accent3: "6366F1",
        },
    ),
    (
        "sunset",
        Theme {
            bg: "150A0F",
            bg2: "2A151D",
            fg: "FFF5EF",
            accent: "FF6A45",
            accent2: "F43F8E",
            accent3: "FBBF24",
        },
    ),
    (
        "forest",
        Theme {
            bg: "06120D",
            bg2: "11241B",
            fg: "ECFDF3",
            accent: "10B981",
            accent2: "A3E635",
            accent3: "06B6D4",
        },
    ),
    (
        "grape",
        Theme {
            bg: "0F0A1C",
            bg2: "1F1733",
            fg: "F6F1FF",
            accent: "8B5CF6",
            accent2: "EC4899",
            accent3: "6366F1",
        },
    ),
    (
        "mono",
        Theme {
            bg: "09090B",
            bg2: "1C1C21",
            fg: "FAFAFA",
            accent: "A1A1AA",
            accent2: "E4E4E7",
            accent3: "52525B",
        },
    ),
    (
        "paper",
        Theme {
            bg: "F7F3EC",
            bg2: "E6DFD3",
            fg: "1C1917",
            accent: "EA580C",
            accent2: "E11D48",
            accent3: "D97706",
        },
    ),
];

/// Accent triads for `color=gradient|random|auto`: one per dark pack, picked
/// by a stable hash of the content so the same URL keeps its colours.
pub(crate) fn pick_seeded(seed: &str) -> &'static Theme {
    const DARK: [&str; 5] = ["dark", "ocean", "sunset", "forest", "grape"];
    let i = super::hash::fnv1a_32(seed.as_bytes()) as usize % DARK.len();
    get(DARK[i]).unwrap_or(&THEMES[0].1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::mark::domain::color::contrast_ratio;

    #[test]
    fn every_theme_ink_clears_aa_on_its_base() {
        for (id, t) in THEMES {
            let r = contrast_ratio(t.fg, t.bg);
            assert!(r >= 7.0, "{id}: ink {r:.2}:1 on its base");
        }
    }

    #[test]
    fn retired_ids_resolve_to_a_curated_pack() {
        for id in [
            "github",
            "tokyonight",
            "radical",
            "gruvbox",
            "dracula",
            "neon",
            "nord",
            "monokai",
        ] {
            assert!(get(id).is_some(), "{id} must still resolve");
            assert!(!list_names().contains(&id), "{id} is not published");
        }
        assert!(get("TokyoNight").is_some());
        assert!(get("not-a-theme").is_none());
    }

    #[test]
    fn light_packs_are_light() {
        assert!(get("light").unwrap().is_light());
        assert!(get("paper").unwrap().is_light());
        assert!(!get("dark").unwrap().is_light());
    }
}
