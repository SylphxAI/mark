//! The Mark — one grammar: form × art × paint × content × geometry × motion.
//!
//! A Mark is a pure function of its URL (ADR-0003): the same spec renders the
//! same SVG forever. No clock, no upstream, no state.
//!
//! UI/UX redesign (ADR-0004): themes are neutral design themes — no personal
//! names, no company names. Content (text/desc) is supplied by the URL, never
//! baked into the product.

/// Geometry family of a mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkForm {
    /// Hero banner — the flagship: art background + layouts + full motion.
    #[default]
    Hero,
    /// Pill — the atomic status mark (shields-style).
    Pill,
    /// Strip — the tech identity row.
    Strip,
    /// Profile — the name + tagline card (text-driven, no baked identities).
    Profile,
    /// Deploy — the conversion pill ("deployed on Sylphx").
    Deploy,
}

impl MarkForm {
    pub const ALL: [&'static str; 5] = ["hero", "pill", "strip", "profile", "deploy"];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Pill => "pill",
            Self::Strip => "strip",
            Self::Profile => "profile",
            Self::Deploy => "deploy",
        }
    }

    /// Unknown forms normalize to the flagship — rendering never fails.
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
            Some("pill") | Some("badge") => Self::Pill,
            Some("strip") | Some("icons") | Some("iconsrow") => Self::Strip,
            Some("profile") | Some("card") => Self::Profile,
            Some("deploy") | Some("deploymark") => Self::Deploy,
            _ => Self::Hero,
        }
    }
}

/// Hero geometry and typography (banner).
#[derive(Debug, Clone, Default)]
pub struct HeroSpec {
    pub layout: Option<String>,
    pub section: Option<String>,
    pub reversal: bool,
    pub font_size: Option<u32>,
    pub desc_size: Option<u32>,
    pub font_color: Option<String>,
    pub font_align: Option<f32>,
    pub font_align_y: Option<f32>,
    pub desc_align: Option<f32>,
    pub desc_align_y: Option<f32>,
    pub rotate: Option<f32>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub text_bg: bool,
}

/// Pill geometry (badge).
#[derive(Debug, Clone, Default)]
pub struct PillSpec {
    pub label: Option<String>,
    pub message: Option<String>,
    pub style: Option<String>,
    pub label_color: Option<String>,
}

/// Strip geometry (icon row).
#[derive(Debug, Clone, Default)]
pub struct StripSpec {
    pub icons: Option<String>,
    pub per_line: Option<u32>,
}

/// Deploy geometry (conversion pill).
#[derive(Debug, Clone, Default)]
pub struct DeploySpec {
    pub service: Option<String>,
}

/// The complete Mark specification — the single grammar surface.
#[derive(Debug, Clone, Default)]
pub struct MarkSpec {
    pub form: MarkForm,
    /// Paint: theme defines the full palette; explicit color is used otherwise.
    pub color: Option<String>,
    pub theme: Option<String>,
    /// Art texture (hero and profile backgrounds).
    pub art: Option<String>,
    /// Content: title line (hero title / profile name).
    pub text: Option<String>,
    /// Content: secondary line (hero desc / profile tagline).
    pub desc: Option<String>,
    /// Content typography: `sans` (default) or `mono`.
    pub font: Option<String>,
    pub credit: bool,
    pub animation: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub hero: HeroSpec,
    pub pill: PillSpec,
    pub strip: StripSpec,
    pub deploy: DeploySpec,
}
