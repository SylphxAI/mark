//! Badge request model and style vocabulary.

#[derive(Debug, Clone)]
pub struct BadgeInput {
    pub label: Option<String>,
    pub message: String,
    pub color: Option<String>,
    pub label_color: Option<String>,
    pub style: BadgeStyle,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeStyle {
    #[default]
    Flat,
    Plastic,
    ForTheBadge,
    Social,
    Pill,
}

impl BadgeStyle {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "plastic" => Self::Plastic,
            "for-the-badge" | "forthebadge" => Self::ForTheBadge,
            "social" => Self::Social,
            "pill" => Self::Pill,
            _ => Self::Flat,
        }
    }
}

/// Shields-compatible named colors + fleet brand colors.
pub fn named_color(c: &str) -> Option<&'static str> {
    Some(match c.to_ascii_lowercase().as_str() {
        "brightgreen" => "4C1",
        "green" => "97CA00",
        "yellow" => "DFB317",
        "yellowgreen" => "A4A61D",
        "orange" => "FE7D37",
        "red" => "E05D44",
        "blue" => "007EC6",
        "lightgrey" | "lightgray" => "9F9F9F",
        "success" => "27AE60",
        "important" => "FE7D37",
        "critical" => "E05D44",
        "informational" => "007EC6",
        "inactive" => "9F9F9F",
        "sylphx" => "D87000",
        "cubeage" => "E03840",
        "epiow" => "7C3AED",
        "ozyrix" => "C9A227",
        _ => return None,
    })
}
