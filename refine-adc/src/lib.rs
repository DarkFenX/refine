//! Adapted data cachers for `refine-core`. They implement persistence for adapted data.
//!
//! ## Feature flags
//!
//! This library provides only one cacher.
//!
//! - `json-zfile`: Enables cacher which writes compressed JSON cache to disk.

#[cfg(feature = "json-zfile")]
pub use json_zfile::JsonZfileAdc;

#[cfg(feature = "json-zfile")]
mod json_zfile;

const VERSION: &str = env!("CARGO_PKG_VERSION");
