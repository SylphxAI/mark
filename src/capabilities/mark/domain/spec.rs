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
    /// Typing — animated typing text (readme-typing-svg geometry).
    Typing,
}

impl MarkForm {
    pub(crate) const ALL: [&'static str; 6] =
        ["hero", "pill", "strip", "profile", "deploy", "typing"];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Pill => "pill",
            Self::Strip => "strip",
            Self::Profile => "profile",
            Self::Deploy => "deploy",
            Self::Typing => "typing",
        }
    }

    /// Unknown forms normalize to the flagship — rendering never fails.
    ///
    /// `identity` is the graph's one `rename-to` id (`MARK-IDENTITY`): those
    /// URLs must reach the profile card, not silently become a hero. Retired
    /// predecessor ids (`badge`, `icons`, `iconsrow`, `card`, `deploymark`)
    /// are unknown input and normalize to hero.
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
            Some("pill") => Self::Pill,
            Some("strip") => Self::Strip,
            Some("profile") | Some("identity") => Self::Profile,
            Some("deploy") => Self::Deploy,
            Some("typing") => Self::Typing,
            _ => Self::Hero,
        }
    }
}

pub use super::typing::TypingSpec;

/// Hero geometry: the layout family only.
///
/// Predecessor capsule-render typography and placement knobs (size, colour,
/// alignment, rotation, stroke, text background, section flip) are leftover and
/// deliberately absent — the URL cannot reach them (`docs/capabilities.md`,
/// `MARK-GRAMMAR`).
#[derive(Debug, Clone, Default)]
pub struct HeroSpec {
    pub layout: Option<String>,
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
    pub typing: TypingSpec,
}
