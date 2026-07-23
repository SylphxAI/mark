//! Banner request model and layout normalization (pure domain).

/// Layout families (composition, not background recipe).
pub const LAYOUTS: &[&str] = &["default", "plate", "signal", "terminal"];

#[derive(Debug, Clone, Default)]
pub struct BannerInput {
    pub type_name: Option<String>,
    pub color: Option<String>,
    pub theme: Option<String>,
    pub section: Option<String>,
    pub reversal: bool,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub text: Option<String>,
    pub desc: Option<String>,
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
    pub animation: Option<String>,
    pub credit: bool,
    pub seed: Option<String>,
    /// Composition: `default` | `plate` | `signal` | `terminal`
    pub layout: Option<String>,
}

pub fn normalize_layout(raw: Option<&str>) -> &'static str {
    match raw
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .as_deref()
    {
        None | Some("default") | Some("center") => "default",
        Some("plate") | Some("product") | Some("card") | Some("oss") => "plate",
        Some("signal") | Some("hero") => "signal",
        Some("terminal") | Some("cli") | Some("mono") => "terminal",
        _ => "default",
    }
}
