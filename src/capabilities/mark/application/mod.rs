//! Mark application layer — pure render use cases (functional core).

mod badge;
mod badge_logo;
mod badge_social;
mod deploy;
mod hero;
mod hero_placed;
mod pill;
mod profile;
mod render;
mod score;
mod strip;
pub mod tiles;
mod typing;

pub(crate) use hero_placed::render_placed;
pub use render::render;
