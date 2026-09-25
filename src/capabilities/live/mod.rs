//! Live capability (`MARK-LIVE`, ADR-0005 decision 5).
//!
//! Consumer outcome: GitHub stats, top-languages, streak, and repository
//! cards plus dynamic GitHub/npm badges, with no user token. This is the only
//! module allowed a clock and upstream calls, and every call is bounded:
//! timeouts, a concurrency cap, an in-memory TTL cache with request
//! coalescing, stale-on-error, and a `200` fallback card.

pub(crate) mod application;
pub(crate) mod domain;
pub(crate) mod interfaces;

pub use application::LiveService;
