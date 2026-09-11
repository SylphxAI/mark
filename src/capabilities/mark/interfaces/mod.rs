//! Mark inbound adapters.

mod http;

pub(crate) use http::{badge_path, mark_default_handler, mark_handler};
