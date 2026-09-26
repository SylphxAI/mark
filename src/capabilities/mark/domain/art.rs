//! The published art vocabulary.
//!
//! A small curated set: each art type is a distinct, designed composition.
//! One enum owns the ids, the catalogue order, and the parse rule, so the
//! renderer's dispatch is exhaustive instead of stringly typed.

use std::fmt;

/// A background art type (`type=` in a mark URL).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// Layered waves drifting along the bottom edge.
    Waving,
    /// Soft coloured light pooling over a dark (or pale) base.
    Aurora,
    /// A full-bleed mesh gradient.
    Mesh,
    /// One beam of light from above.
    Spotlight,
    /// A fine grid fading out from a soft centre glow.
    Grid,
    /// A flat card with one accent hairline.
    Minimal,
    /// A terminal window.
    Terminal,
    /// Text only.
    Transparent,
}

impl Art {
    /// Catalogue order (published as `art_types`; the studio shows it too).
    pub const ALL: [Art; 8] = [
        Art::Waving,
        Art::Aurora,
        Art::Mesh,
        Art::Spotlight,
        Art::Grid,
        Art::Minimal,
        Art::Terminal,
        Art::Transparent,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Art::Waving => "waving",
            Art::Aurora => "aurora",
            Art::Mesh => "mesh",
            Art::Spotlight => "spotlight",
            Art::Grid => "grid",
            Art::Minimal => "minimal",
            Art::Terminal => "terminal",
            Art::Transparent => "transparent",
        }
    }

    /// Parse `type=`. Ids published before the curated set map to the
    /// closest survivor, so every banner ever embedded keeps a designed look;
    /// an unknown name is the default (`waving`).
    pub fn parse(raw: &str) -> Art {
        match raw.trim().to_ascii_lowercase().as_str() {
            "aurora" | "void" | "firefly" | "meteor" | "constellation" | "orbit" | "ring"
            | "pulse" | "neon" => Art::Aurora,
            "mesh" | "plasma" | "holo" | "liquid" | "iridescent" | "silk" | "prism" | "noise"
            | "blur" => Art::Mesh,
            "spotlight" | "glass" | "beam" | "horizon" | "dusk" => Art::Spotlight,
            "grid" | "circuit" | "hud" | "checkered" => Art::Grid,
            "minimal" | "rect" | "soft" | "rounded" | "product" | "oss" | "org" => Art::Minimal,
            "terminal" => Art::Terminal,
            "transparent" => Art::Transparent,
            _ => Art::Waving,
        }
    }
}

impl fmt::Display for Art {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

/// Catalogue ids in published order.
pub(crate) fn art_ids() -> Vec<&'static str> {
    Art::ALL.iter().map(|a| a.id()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_id_round_trips_and_is_unique() {
        let mut seen = std::collections::HashSet::new();
        for art in Art::ALL {
            assert!(seen.insert(art.id()), "duplicate id {}", art.id());
            assert_eq!(Art::parse(art.id()), art, "round trip for {}", art.id());
        }
    }

    #[test]
    fn retired_ids_map_to_the_closest_curated_art() {
        for (old, new) in [
            ("plasma", Art::Mesh),
            ("constellation", Art::Aurora),
            ("glass", Art::Spotlight),
            ("circuit", Art::Grid),
            ("wave", Art::Waving),
            ("shark", Art::Waving),
            ("soft", Art::Minimal),
            ("product", Art::Minimal),
        ] {
            assert_eq!(Art::parse(old), new, "{old}");
        }
    }

    #[test]
    fn unknown_and_case_variants_normalize() {
        assert_eq!(Art::parse("not-a-type"), Art::Waving);
        assert_eq!(Art::parse("MESH"), Art::Mesh);
        assert_eq!(Art::parse("  aurora "), Art::Aurora);
    }
}
