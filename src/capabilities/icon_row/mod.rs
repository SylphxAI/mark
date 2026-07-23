//! Icon-row capability — tech stack icon strips as SVG.
//!
//! Consumer outcome: a list of technology ids becomes a compact icon row.

pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::render_row;
pub use domain::available;
