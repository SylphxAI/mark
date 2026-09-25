//! Icon vocabulary: resolve any public icon id to a glyph, and paint it.
//!
//! Resolution order for an id (case-insensitive):
//! 1. skill-icons and Mark short names → a skill-icons name,
//! 2. a skill-icons name → its [`Target`] (Simple Icons slug, hand glyph, or
//!    lettermark),
//! 3. any Simple Icons slug, title, or alias ([`brand_icons::lookup`]).
//!
//! The hand-drawn glyphs below cover brands Simple Icons no longer ships.

use crate::capabilities::mark::domain::brand_icons::{self, BrandIcon};
use crate::capabilities::mark::domain::color::{contrast_ratio, relative_luminance};
use crate::capabilities::mark::domain::icon_aliases::{vocabulary, Target};
use crate::capabilities::mark::domain::svg::esc;

/// How a resolved icon is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IconArt {
    /// Simple Icons path on a 24×24 viewBox.
    Brand(BrandIcon),
    /// Hand-drawn markup on a 32×32 box, painted with `currentColor`.
    Hand(&'static str),
    /// A short lettermark.
    Mono(&'static str),
}

/// A resolved icon: its art and brand color (six hex digits, no `#`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Icon {
    pub art: IconArt,
    pub hex: &'static str,
}

/// Minimum WCAG contrast between a brand color and its tile before the glyph
/// switches to a neutral ink.
/// 2.0 keeps mid-tone brands (Vue, Tailwind, Photoshop blue) in color on the
/// light tile while ink-dark (GitHub, Rust) and pale (JavaScript yellow on
/// light) brands switch.
pub(crate) const MIN_BRAND_CONTRAST: f64 = 2.0;

pub(crate) fn glyph(id: &str) -> Option<&'static str> {
    Some(match id {
        "aws" => "<path d=\"M6 18 Q16 26 26 18\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"14\" text-anchor=\"middle\" font-size=\"9\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"currentColor\">aws</text>",
        "azure" => "<path fill=\"currentColor\" d=\"M12.4 5 H19.2 L10.4 27 H3.6 Z M20.6 9.6 L28.4 27 H13.4 L19.8 24.2 L15.2 18.4 Z\"/>",
        "java" => "<ellipse cx=\"16\" cy=\"20\" rx=\"8\" ry=\"6\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M24 18 Q28 20 24 22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><path d=\"M12 8 C16 6 20 10 16 14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\"/>",
        "csharp" => "<rect x=\"6\" y=\"6\" width=\"20\" height=\"20\" rx=\"4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\"/><text x=\"16\" y=\"21\" text-anchor=\"middle\" font-size=\"11\" font-weight=\"700\" font-family=\"sans-serif\" fill=\"currentColor\">C#</text>",
        "vscode" => "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M22 4 L28 7 V25 L22 28 L11.5 18.2 L6.3 22.2 L4 21 V11 L6.3 9.8 L11.5 13.8 Z M22 10.6 L15.2 16 L22 21.4 Z M6.8 13.2 V18.8 L9.6 16 Z\"/>",
        "windows" => "<path fill=\"currentColor\" d=\"M5 5 H15.4 V15.4 H5 Z M16.6 5 H27 V15.4 H16.6 Z M5 16.6 H15.4 V27 H5 Z M16.6 16.6 H27 V27 H16.6 Z\"/>",
        _ => return None,
    })
}

/// Featured ids for the catalog and studio (every one resolves).
pub(crate) fn available() -> Vec<&'static str> {
    [
        "rust",
        "go",
        "ts",
        "js",
        "python",
        "react",
        "node",
        "docker",
        "kubernetes",
        "linux",
        "git",
        "github",
        "postgres",
        "redis",
        "aws",
        "gcp",
        "azure",
        "nextjs",
        "vue",
        "svelte",
        "bun",
        "deno",
        "css",
        "html",
        "graphql",
        "tailwind",
        "prisma",
        "sqlite",
        "nginx",
        "cloudflare",
        "vercel",
        "java",
        "kotlin",
        "csharp",
        "php",
        "ruby",
        "elixir",
        "zig",
        "terraform",
        "helm",
        "mongodb",
        "mysql",
        "prometheus",
        "grafana",
        "flutter",
        "swift",
        "ansible",
    ]
    .to_vec()
}

/// Every skill-icons id: full names first (skill-icons order), then short names.
pub(crate) fn skill_ids() -> impl Iterator<Item = &'static str> {
    let v = vocabulary();
    v.icons
        .iter()
        .map(|(name, _)| *name)
        .chain(v.skill_short.iter().map(|(short, _)| *short))
}

/// skill-icons full names in skill-icons order (the `i=all` set).
pub(crate) fn skill_names() -> impl Iterator<Item = &'static str> {
    vocabulary().icons.iter().map(|(name, _)| *name)
}

/// Distinct glyphs: every Simple Icons brand plus the skill-icons names drawn
/// by hand or as a lettermark.
pub(crate) fn count() -> usize {
    let extra = vocabulary()
        .icons
        .iter()
        .filter(|(_, t)| !matches!(t, Target::Brand(_)))
        .count();
    brand_icons::count() + extra
}

/// Resolve any public icon id; `None` for an unknown id.
pub(crate) fn resolve(raw: &str) -> Option<Icon> {
    let id = raw.trim().to_ascii_lowercase();
    if id.is_empty() {
        return None;
    }
    let v = vocabulary();
    let name = v
        .skill_short
        .iter()
        .chain(&v.mark_short)
        .find(|(short, _)| *short == id)
        .map_or(id.as_str(), |(_, name)| *name);
    if let Some((_, target)) = v.icons.iter().find(|(n, _)| *n == name) {
        return from_target(*target);
    }
    brand_icons::lookup(raw).map(brand)
}

fn brand(icon: BrandIcon) -> Icon {
    Icon {
        art: IconArt::Brand(icon),
        hex: icon.hex,
    }
}

fn from_target(target: Target) -> Option<Icon> {
    match target {
        Target::Brand(slug) => brand_icons::lookup(slug).map(brand),
        Target::Hand(id, hex) => glyph(id).map(|markup| Icon {
            art: IconArt::Hand(markup),
            hex,
        }),
        Target::Mono(text, hex) => Some(Icon {
            art: IconArt::Mono(text),
            hex,
        }),
    }
}

/// `#rrggbb` paint for a brand color on `bg`: the brand color when it reads
/// (WCAG contrast ≥ [`MIN_BRAND_CONTRAST`]), else white on a dark tile and
/// near-black on a light one.
pub(crate) fn brand_paint(hex: &str, bg: &str) -> String {
    if contrast_ratio(hex, bg) >= MIN_BRAND_CONTRAST {
        format!("#{hex}")
    } else if relative_luminance(bg) < 0.4 {
        "#FFFFFF".into()
    } else {
        "#1F2328".into()
    }
}

/// Glyph markup filling the `size`×`size` box at the origin, painted `fill`.
pub(crate) fn paint(icon: &Icon, size: f64, fill: &str) -> String {
    match icon.art {
        IconArt::Brand(b) => format!(
            "<path transform=\"scale({})\" fill=\"{fill}\" d=\"{}\"/>",
            num(size / 24.0),
            b.path
        ),
        // Hand glyphs draw inside 4..28 of their 32 box.
        IconArt::Hand(markup) => format!(
            "<g transform=\"translate({o},{o}) scale({k})\" color=\"{fill}\">{markup}</g>",
            o = num(-size / 6.0),
            k = num(size / 24.0)
        ),
        IconArt::Mono(text) => {
            let ratio = match text.chars().count() {
                1 => 0.8,
                2 => 0.62,
                _ => 0.46,
            };
            format!(
                "<text x=\"{c}\" y=\"{c}\" dy=\"0.35em\" text-anchor=\"middle\" \
                 font-family=\"ui-sans-serif,system-ui,-apple-system,'Segoe UI',Helvetica,Arial,sans-serif\" \
                 font-weight=\"800\" font-size=\"{fs}\" letter-spacing=\"-0.02em\" fill=\"{fill}\">{}</text>",
                esc(text),
                c = num(size / 2.0),
                fs = num(size * ratio)
            )
        }
    }
}

/// Compact decimal for SVG attributes (at most four places, no trailing zeros).
pub(crate) fn num(v: f64) -> String {
    let s = format!("{v:.4}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" {
        "0".into()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slug(id: &str) -> Option<&'static str> {
        match resolve(id)?.art {
            IconArt::Brand(b) => Some(b.slug),
            _ => None,
        }
    }

    #[test]
    fn every_skill_icons_id_resolves() {
        for id in skill_ids().chain(vocabulary().mark_short.iter().map(|(s, _)| *s)) {
            assert!(
                resolve(id).is_some(),
                "skill-icons id {id} does not resolve"
            );
        }
        assert!(skill_names().count() > 230);
    }

    #[test]
    fn every_featured_id_resolves() {
        for id in available() {
            assert!(resolve(id).is_some(), "featured id {id} does not resolve");
        }
    }

    #[test]
    fn short_names_and_slugs_resolve_to_brands() {
        assert_eq!(slug("js"), Some("javascript"));
        assert_eq!(slug("TS"), Some("typescript"));
        assert_eq!(slug("k8s"), Some("kubernetes"));
        assert_eq!(slug("nodejs"), Some("nodedotjs"));
        assert_eq!(slug("node"), Some("nodedotjs"));
        assert_eq!(slug("tailwind"), Some("tailwindcss"));
        assert_eq!(slug("postgres"), Some("postgresql"));
        assert_eq!(slug("go"), Some("go"));
        assert_eq!(slug("html"), Some("html5"));
        // Any Simple Icons slug or title, beyond the skill-icons vocabulary.
        assert_eq!(slug("fastify"), Some("fastify"));
        assert_eq!(slug("Next.js"), Some("nextdotjs"));
        assert!(resolve("not-an-icon").is_none());
        assert!(resolve(" ").is_none());
    }

    #[test]
    fn removed_brands_fall_back_to_hand_glyphs_and_lettermarks() {
        for id in [
            "cs", "c#", "csharp", "vscode", "aws", "azure", "java", "windows",
        ] {
            assert!(
                matches!(resolve(id).unwrap().art, IconArt::Hand(_)),
                "{id} should use a hand glyph"
            );
        }
        assert_eq!(resolve("ps").unwrap().art, IconArt::Mono("Ps"));
        assert_eq!(resolve("linkedin").unwrap().art, IconArt::Mono("in"));
    }

    #[test]
    fn brand_paint_keeps_readable_brand_colors_only() {
        assert_eq!(brand_paint("F7DF1E", "#242938"), "#F7DF1E");
        assert_eq!(brand_paint("181717", "#242938"), "#FFFFFF");
        assert_eq!(brand_paint("FFFFFF", "#F4F2ED"), "#1F2328");
        assert_eq!(brand_paint("3178C6", "#F4F2ED"), "#3178C6");
        assert_eq!(brand_paint("4FC08D", "#F4F2ED"), "#4FC08D");
        assert_eq!(brand_paint("F7DF1E", "#F4F2ED"), "#1F2328");
    }

    #[test]
    fn num_is_compact() {
        assert_eq!(num(1.0), "1");
        assert_eq!(num(7.0 / 6.0), "1.1667");
        assert_eq!(num(-0.00001), "0");
        assert_eq!(num(160.5), "160.5");
    }
}
