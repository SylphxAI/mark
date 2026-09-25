//! Pill application: pure MarkSpec → pill SVG (the atomic status mark).
//!
//! The pill is the shields badge: geometry is badge-maker's for the chosen
//! style; paint comes from the shared grammar (theme defines the palette,
//! explicit color wins otherwise); motion applies at text level.

use super::badge::{compose, Badge};
use crate::capabilities::mark::domain::MarkSpec;

/// Pill form entry: paint and text come from the shared MarkSpec grammar.
pub fn render(spec: &MarkSpec) -> String {
    let message = match spec.pill.message.as_deref() {
        None | Some("") => "ok",
        Some(m) => m,
    };
    compose(&Badge::from_spec(spec, message, "4A90E2"))
}
