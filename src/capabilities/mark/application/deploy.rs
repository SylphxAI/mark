//! Deploy application: pure MarkSpec → the conversion pill.

use crate::capabilities::mark::application::pill::render_pill;
use crate::capabilities::mark::domain::{
    cap_text, MarkSpec, PillStyle, MAX_SERVICE_CHARS,
};

pub fn render(spec: &MarkSpec) -> String {
    let service = cap_text(
        spec.deploy.service.as_deref().unwrap_or("Sylphx"),
        MAX_SERVICE_CHARS,
    );
    let message = if service.is_empty() {
        "Sylphx".into()
    } else {
        format!("{service} · Sylphx")
    };
    render_pill(
        "deployed on",
        &message,
        spec.color.as_deref().or(Some("sylphx")),
        Some("1A1A2E"),
        PillStyle::parse(spec.pill.style.as_deref().unwrap_or("flat")),
        spec.theme.as_deref(),
        spec.animation.as_deref(),
    )
}
