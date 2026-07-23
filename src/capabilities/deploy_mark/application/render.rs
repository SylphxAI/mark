//! Deploy-mark application: promotional "deployed on Sylphx" badge.

use crate::capabilities::badge::{self, BadgeInput, BadgeStyle};

/// Render a static deploy promo badge.
pub fn render(service: &str, theme: Option<&str>, style: &str) -> String {
    badge::render(&BadgeInput {
        label: Some("deployed on".into()),
        message: if service.is_empty() {
            "Sylphx".into()
        } else {
            format!("{service} · Sylphx")
        },
        color: Some("sylphx".into()),
        label_color: Some("1A1A2E".into()),
        style: BadgeStyle::parse(style),
        theme: theme.map(|s| s.to_string()),
    })
}
