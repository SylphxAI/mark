//! Mark inbound adapters.

pub(crate) mod dialects;
mod http;

pub(crate) use http::{
    badge_path, icons_handler, mark_default_handler, mark_handler, static_v1, typing_handler,
};
