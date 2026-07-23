//! Banner domain: catalogs, request model, motion, and pure shape generation.

mod input;
pub mod motion;
pub mod shapes;

pub use input::{normalize_layout, BannerInput, LAYOUTS};
pub use motion::{
    ambient_gain, normalize_animation, text_children, text_open_attrs, ANIMATIONS,
};
pub use shapes::{
    is_banner_type, normalize_type, shape_background, shape_defs, BANNER_TYPES, FEATURED_TYPES,
};
