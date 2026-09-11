//! The published art vocabulary.
//!
//! One enum owns the ids, the catalogue order, and the parse rule, so the
//! renderer's dispatch can be exhaustive instead of stringly typed.

use std::fmt;

/// A background art type (`type=` in a mark URL).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Transparent,
    Plasma,
    Holo,
    Neon,
    Meteor,
    Liquid,
    Prism,
    Void,
    Firefly,
    Silk,
    Iridescent,
    Rect,
    Soft,
    Rounded,
    Aurora,
    Mesh,
    Glass,
    Horizon,
    Dusk,
    Wave,
    Waving,
    Orbit,
    Ring,
    Beam,
    Terminal,
    Constellation,
    Blur,
    Grid,
    Circuit,
    Hud,
    Pulse,
    Noise,
    Cylinder,
    Slice,
    Egg,
    Shark,
    Venom,
    Speech,
    Checkered,
    Product,
    Oss,
    Org,
}

impl Art {
    /// Catalogue order (published as `art_types`).
    pub const ALL: [Art; 42] = [
        Art::Transparent,
        Art::Plasma,
        Art::Holo,
        Art::Neon,
        Art::Meteor,
        Art::Liquid,
        Art::Prism,
        Art::Void,
        Art::Firefly,
        Art::Silk,
        Art::Iridescent,
        Art::Rect,
        Art::Soft,
        Art::Rounded,
        Art::Aurora,
        Art::Mesh,
        Art::Glass,
        Art::Horizon,
        Art::Dusk,
        Art::Wave,
        Art::Waving,
        Art::Orbit,
        Art::Ring,
        Art::Beam,
        Art::Terminal,
        Art::Constellation,
        Art::Blur,
        Art::Grid,
        Art::Circuit,
        Art::Hud,
        Art::Pulse,
        Art::Noise,
        Art::Cylinder,
        Art::Slice,
        Art::Egg,
        Art::Shark,
        Art::Venom,
        Art::Speech,
        Art::Checkered,
        Art::Product,
        Art::Oss,
        Art::Org,
    ];

    /// Studio showcase order (published as `featured_art_types`).
    pub const FEATURED: [Art; 18] = [
        Art::Wave,
        Art::Waving,
        Art::Soft,
        Art::Rounded,
        Art::Rect,
        Art::Slice,
        Art::Glass,
        Art::Product,
        Art::Terminal,
        Art::Aurora,
        Art::Mesh,
        Art::Plasma,
        Art::Holo,
        Art::Neon,
        Art::Liquid,
        Art::Silk,
        Art::Orbit,
        Art::Constellation,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Art::Transparent => "transparent",
            Art::Plasma => "plasma",
            Art::Holo => "holo",
            Art::Neon => "neon",
            Art::Meteor => "meteor",
            Art::Liquid => "liquid",
            Art::Prism => "prism",
            Art::Void => "void",
            Art::Firefly => "firefly",
            Art::Silk => "silk",
            Art::Iridescent => "iridescent",
            Art::Rect => "rect",
            Art::Soft => "soft",
            Art::Rounded => "rounded",
            Art::Aurora => "aurora",
            Art::Mesh => "mesh",
            Art::Glass => "glass",
            Art::Horizon => "horizon",
            Art::Dusk => "dusk",
            Art::Wave => "wave",
            Art::Waving => "waving",
            Art::Orbit => "orbit",
            Art::Ring => "ring",
            Art::Beam => "beam",
            Art::Terminal => "terminal",
            Art::Constellation => "constellation",
            Art::Blur => "blur",
            Art::Grid => "grid",
            Art::Circuit => "circuit",
            Art::Hud => "hud",
            Art::Pulse => "pulse",
            Art::Noise => "noise",
            Art::Cylinder => "cylinder",
            Art::Slice => "slice",
            Art::Egg => "egg",
            Art::Shark => "shark",
            Art::Venom => "venom",
            Art::Speech => "speech",
            Art::Checkered => "checkered",
            Art::Product => "product",
            Art::Oss => "oss",
            Art::Org => "org",
        }
    }

    /// Parse `type=`; an unknown name is the restrained default (`waving`).
    pub fn parse(raw: &str) -> Art {
        match raw.trim().to_ascii_lowercase().as_str() {
            "transparent" => Art::Transparent,
            "plasma" => Art::Plasma,
            "holo" => Art::Holo,
            "neon" => Art::Neon,
            "meteor" => Art::Meteor,
            "liquid" => Art::Liquid,
            "prism" => Art::Prism,
            "void" => Art::Void,
            "firefly" => Art::Firefly,
            "silk" => Art::Silk,
            "iridescent" => Art::Iridescent,
            "rect" => Art::Rect,
            "soft" => Art::Soft,
            "rounded" => Art::Rounded,
            "aurora" => Art::Aurora,
            "mesh" => Art::Mesh,
            "glass" => Art::Glass,
            "horizon" => Art::Horizon,
            "dusk" => Art::Dusk,
            "wave" => Art::Wave,
            "waving" => Art::Waving,
            "orbit" => Art::Orbit,
            "ring" => Art::Ring,
            "beam" => Art::Beam,
            "terminal" => Art::Terminal,
            "constellation" => Art::Constellation,
            "blur" => Art::Blur,
            "grid" => Art::Grid,
            "circuit" => Art::Circuit,
            "hud" => Art::Hud,
            "pulse" => Art::Pulse,
            "noise" => Art::Noise,
            "cylinder" => Art::Cylinder,
            "slice" => Art::Slice,
            "egg" => Art::Egg,
            "shark" => Art::Shark,
            "venom" => Art::Venom,
            "speech" => Art::Speech,
            "checkered" => Art::Checkered,
            "product" => Art::Product,
            "oss" => Art::Oss,
            "org" => Art::Org,
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

/// Studio showcase ids in published order.
pub(crate) fn featured_art_ids() -> Vec<&'static str> {
    Art::FEATURED.iter().map(|a| a.id()).collect()
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
        assert_eq!(seen.len(), 42);
    }

    #[test]
    fn unknown_and_case_variants_normalize() {
        assert_eq!(Art::parse("not-a-type"), Art::Waving);
        assert_eq!(Art::parse("PLASMA"), Art::Plasma);
        assert_eq!(Art::parse("  neon "), Art::Neon);
    }

    #[test]
    fn featured_is_a_subset_of_all() {
        for art in Art::FEATURED {
            assert!(
                Art::ALL.contains(&art),
                "featured {} not in catalogue",
                art.id()
            );
        }
    }
}
