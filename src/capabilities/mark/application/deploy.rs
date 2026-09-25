//! Deploy application: conversion mark ("deployed on Sylphx").
//!
//! Same grammar and geometry as the pill (style / theme / color / motion /
//! font) with the deploy mark drawn as the label-side logo, so the conversion
//! surface is recognisable but sits among shields badges without friction.

use super::badge::{compose, Badge, Logo};
use crate::capabilities::mark::domain::{cap_text, MarkSpec, MAX_SERVICE_CHARS};

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
    let mut spec = spec.clone();
    spec.pill.label = Some("deployed on".into());
    spec.pill.label_color = Some("1A1A2E".into());
    spec.pill.logo = None;
    let mut badge = Badge::from_spec(&spec, &message, "D87000");
    badge.logo = Some(Logo::DeployMark);
    compose(&badge)
}
