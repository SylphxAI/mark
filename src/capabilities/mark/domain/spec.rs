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
    /// Score — a pill whose message carries a graded value and progress ring.
    Score,
}

impl MarkForm {
    pub(crate) const ALL: [&'static str; 7] = [
        "hero", "pill", "strip", "profile", "deploy", "typing", "score",
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Pill => "pill",
            Self::Strip => "strip",
            Self::Profile => "profile",
            Self::Deploy => "deploy",
            Self::Typing => "typing",
            Self::Score => "score",
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
            Some("score") => Self::Score,
            _ => Self::Hero,
        }
    }
}

use super::shapes::capsule::Silhouette;
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

/// Placed typography for one text role of a dialect hero.
#[derive(Debug, Clone)]
pub(crate) struct PlacedText {
    pub size: u32,
    /// Canonical `#hex` token.
    pub color: String,
    pub weight: u32,
    /// Horizontal center per line, % of width (missing lines reuse the first).
    pub x: Vec<f32>,
    /// Vertical center per line, % of height (missing lines step down).
    pub y: Vec<f32>,
    /// Line step in em when a line has no `y` of its own.
    pub step_em: f32,
}

/// Dialect-only hero overrides: the typography, placement, and silhouette
/// knobs of the capsule-render dialect (ADR-0005 decision 4).
///
/// Crate-private and absent from [`MarkSpec`], so the native grammar
/// (`/api/v1/mark/…`) cannot reach them: only the capsule dialect module
/// builds one, and `hero::render_placed` paints it.
#[derive(Debug, Clone)]
pub(crate) struct HeroOverrides {
    pub title: PlacedText,
    pub desc: PlacedText,
    /// A `font-family` value already built by `text::requested_family`.
    pub family: String,
    /// Text rotation about the canvas center, degrees.
    pub rotate: f32,
    /// Title outline: canonical `#hex` color and width.
    pub stroke: Option<(String, f32)>,
    /// Rounded plate behind the first title line: canonical `#hex`.
    pub text_bg: Option<String>,
    /// Rotate the silhouette 180° (a footer banner).
    pub flip: bool,
    /// Mirror the silhouette horizontally.
    pub mirror: bool,
    /// Text motion: a published animation id or a dialect-only one.
    pub motion: &'static str,
    pub silhouette: Silhouette,
    /// Paint stops `(offset %, color)`; one stop is a solid.
    pub paint: Vec<(f32, String)>,
}

/// Pill geometry (badge).
#[derive(Debug, Clone, Default)]
pub struct PillSpec {
    pub label: Option<String>,
    pub message: Option<String>,
    pub style: Option<String>,
    pub label_color: Option<String>,
    /// Logo left of the label: a Simple Icons slug or a base64 image data URI.
    pub logo: Option<String>,
    /// Paint for a named logo.
    pub logo_color: Option<String>,
    /// `auto` widens wide logos (shields `logoSize`).
    pub logo_size: Option<String>,
    /// Explicit logo width in px (shields `logoWidth`).
    pub logo_width: Option<u32>,
}

/// Score geometry: a value out of a maximum.
#[derive(Debug, Clone, Default)]
pub struct ScoreSpec {
    pub value: Option<f64>,
    pub max: Option<f64>,
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
    pub score: ScoreSpec,
}
