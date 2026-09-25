//! Badge logos: what sits left of the label.
//!
//! shields semantics: `logo=` is a base64 image data URI or a Simple Icons
//! name (slug, title, or alias). A named logo paints in its brand color,
//! except that a dark brand on a regular badge paints `whitesmoke` and a
//! bright brand on the social style paints `#333`; `logoColor` overrides.
//! Unknown names draw no logo. Named glyphs are painted inline from the
//! embedded registry (no nested image), 14 px high, at the badge-maker slot.

use crate::capabilities::mark::domain::brand_icons::{self, BrandIcon};
use crate::capabilities::mark::domain::paint::{brightness, css_color};
use crate::capabilities::mark::domain::pill::fmt;
use crate::capabilities::mark::domain::shields::data_uri_logo;
use crate::capabilities::mark::domain::{MarkSpec, PillStyle};

/// Logo height in px (badge-maker `DEFAULT_LOGO_HEIGHT`).
pub(super) const LOGO_H: f64 = 14.0;

/// What sits left of the label.
#[derive(Debug, Clone)]
pub(crate) enum Logo {
    /// A validated base64 image data URI.
    Image { href: String, width: f64 },
    /// A Simple Icons glyph (24×24 path) in a resolved `#hex` paint.
    Icon {
        icon: BrandIcon,
        fill: String,
        width: f64,
    },
    /// The deploy conversion mark: a dot in a ring, painted as vectors.
    DeployMark,
}

impl Logo {
    /// Resolve `logo`/`logoColor`/`logoWidth` for a style; `None` when absent
    /// or unknown.
    pub(super) fn from_spec(spec: &MarkSpec, style: PillStyle) -> Option<Self> {
        let raw = spec.pill.logo.as_deref()?.trim();
        let width = spec.pill.logo_width.unwrap_or(14).clamp(1, 64) as f64;
        if let Some(href) = data_uri_logo(raw) {
            return Some(Self::Image { href, width });
        }
        let icon = brand_icons::lookup(&shields_alias(raw).replace(' ', "-"))
            .or_else(|| brand_icons::lookup(raw))?;
        let fill = spec
            .pill
            .logo_color
            .as_deref()
            .and_then(css_color)
            .unwrap_or_else(|| default_fill(&icon, style));
        Some(Self::Icon { icon, fill, width })
    }

    pub(super) fn width(&self) -> f64 {
        match self {
            Self::Image { width, .. } | Self::Icon { width, .. } => *width,
            Self::DeployMark => LOGO_H,
        }
    }

    pub(super) fn paint(&self, x: f64, h: f64) -> String {
        let y = (h - LOGO_H) / 2.0;
        match self {
            Self::Image { href, width } => format!(
                "<image x=\"{}\" y=\"{}\" width=\"{}\" height=\"14\" href=\"{href}\"/>",
                fmt(x),
                fmt(y),
                fmt(*width)
            ),
            Self::Icon { icon, fill, width } => {
                // Centre the 14 px square glyph in its slot (a wider
                // `logoWidth` letterboxes like an <image> would).
                let gx = x + (width - LOGO_H) / 2.0;
                format!(
                    "<path transform=\"translate({} {}) scale({})\" fill=\"{fill}\" d=\"{}\"/>",
                    fmt(gx),
                    fmt(y),
                    LOGO_H / 24.0,
                    icon.path
                )
            }
            Self::DeployMark => {
                let (cx, cy) = (fmt(x + 7.0), fmt(h / 2.0));
                format!(
                    "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"2.5\" fill=\"#fff\"/>\
                     <circle cx=\"{cx}\" cy=\"{cy}\" r=\"5.75\" fill=\"none\" stroke=\"#fff\" stroke-width=\"1.5\"/>"
                )
            }
        }
    }
}

/// shields' `logoAliases` for logos Simple Icons renamed or removed.
fn shields_alias(name: &str) -> &str {
    match name.to_ascii_lowercase().as_str() {
        "azuredevops" | "tfs" => "azure-devops",
        "eclipse" => "eclipse-ide",
        "gitter-white" => "gitter",
        "scrutinizer" => "scrutinizer-ci",
        "stackoverflow" => "stack-overflow",
        "travis" => "travisci",
        _ => name,
    }
}

/// shields `getSimpleIconStyle`: brand color, with a light fallback for dark
/// brands and a dark one for bright brands on the social style.
fn default_fill(icon: &BrandIcon, style: PillStyle) -> String {
    let hex = format!("#{}", icon.hex.to_ascii_lowercase());
    let b = brightness(&hex);
    match style {
        PillStyle::Social if b >= 0.6 => "#333333".into(),
        PillStyle::Social => hex,
        _ if b <= 0.4 => "#f5f5f5".into(),
        _ => hex,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::mark::domain::PillSpec;

    fn logo(name: &str, color: Option<&str>, style: PillStyle) -> Option<Logo> {
        let spec = MarkSpec {
            pill: PillSpec {
                logo: Some(name.into()),
                logo_color: color.map(Into::into),
                ..Default::default()
            },
            ..Default::default()
        };
        Logo::from_spec(&spec, style)
    }

    fn fill(l: Option<Logo>) -> String {
        match l {
            Some(Logo::Icon { fill, .. }) => fill,
            other => panic!("expected icon, got {other:?}"),
        }
    }

    #[test]
    fn named_logos_follow_shields_paint() {
        // Rust's brand is black: whitesmoke on a regular badge.
        assert_eq!(fill(logo("rust", None, PillStyle::Flat)), "#f5f5f5");
        assert_eq!(fill(logo("rust", None, PillStyle::Social)), "#000000");
        // A bright brand keeps its color, and turns #333 on social.
        assert_eq!(fill(logo("npm", None, PillStyle::Flat)), "#f5f5f5");
        assert_eq!(fill(logo("javascript", None, PillStyle::Flat)), "#f7df1e");
        assert_eq!(fill(logo("javascript", None, PillStyle::Social)), "#333333");
        assert_eq!(
            fill(logo("rust", Some("orange"), PillStyle::Flat)),
            "#ea7233"
        );
        assert_eq!(
            fill(logo("Node.js", Some("white"), PillStyle::Flat)),
            "#ffffff"
        );
        assert!(logo("not-a-real-brand-xyz", None, PillStyle::Flat).is_none());
    }
}
