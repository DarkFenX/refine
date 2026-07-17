#![feature(integer_casts)]

//! EVE data handlers which use JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source. They enable `refine-core`
//! library to access EVE data and process it for its needs.
//!
//! ## Feature flags
//!
//! This library provides two different data handlers, with their availability controlled via
//! feature flags to reduce the size of compiled code.
//!
//! - `full`: Enables both handlers.
//! - `phb-http`: Enables handler which fetches data over HTTP.
//! - `phb-fs`: Enables handler which reads data from filesystem.

#[cfg(feature = "phb-fs")]
pub use phb::PhbFilesystemEdh;
#[cfg(feature = "phb-http")]
pub use phb::{PhbHttpEdh, PhbHttpEdhInitError};

#[cfg(any(feature = "phb-http", feature = "phb-fs"))]
mod phb;
