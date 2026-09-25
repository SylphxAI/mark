//! Live application: upstream port and adapters, caches, and the service.

pub(crate) mod cache;
mod fixtures;
mod github;
pub(crate) mod npm;
mod service;
mod upstream;

pub use service::LiveService;
