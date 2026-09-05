//! Mark capability — the one concept (ADR-0003).
//!
//! Consumer outcome: URL parameters become a beautiful, deterministic, branded
//! SVG mark — hero, pill, strip, profile, or deploy — from one grammar.

pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::render;
pub use domain::{parse_public_mark_url, readme_markdown_embed, MarkForm, MarkSpec, StudioBoot};
