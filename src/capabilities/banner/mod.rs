//! Banner capability — artistic README/header/footer SVG marks.
//!
//! Consumer outcome: URL parameters become a cacheable SVG banner with
//! layout, chromatic system, and optional SMIL motion.

pub mod application;
pub mod domain;
pub mod interfaces;

pub use application::render;
pub use domain::{
    is_banner_type, normalize_animation, normalize_layout, normalize_type, ANIMATIONS,
    BANNER_TYPES, FEATURED_TYPES, LAYOUTS, BannerInput,
};
