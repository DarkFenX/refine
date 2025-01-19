//! Adapted data handlers for `reefast-core`. They serve adapted data types to the library, and
//! implement persistent cache when applicable.
//!
//! ## Feature flags
//!
//! This library provides two different data handlers, with their availability controlled via
//! feature flags to reduce the amount of compiled code.
//!
//! - `full`: Enables both handlers.
//! - `json`: Enables handler which stores data in RAM, and writes persistent JSON cache to disk.
//! - `ram`: Enables handler which stores data in RAM, and does not implement persistent cache.

#[cfg(feature = "json")]
pub use handler_json::RamJsonAdh;
#[cfg(feature = "ram")]
pub use handler_ram::RamOnlyAdh;
pub use util::Error;

#[cfg(feature = "json")]
mod handler_json;
#[cfg(feature = "ram")]
mod handler_ram;
mod util;

const VERSION: &str = env!("CARGO_PKG_VERSION");
