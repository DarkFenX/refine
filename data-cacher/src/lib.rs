//! Adapted data cachers for `reefast-core`. They implement persistent for adapted data.
//!
//! ## Feature flags
//!
//! This library provides only one cacher.
//!
//! - `full`: Enables all available cachers.
//! - `json-zfile`: Enables cacher which writes compressed JSON cache to disk.

#[cfg(feature = "json-zfile")]
pub use cacher_json::JsonZfileAdc;
pub use util::Error;

#[cfg(feature = "json-zfile")]
mod cacher_json;
mod util;

const VERSION: &str = env!("CARGO_PKG_VERSION");
