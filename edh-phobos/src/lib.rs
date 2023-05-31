//! EVE data handlers which use JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source. They enable `reefast-core`
//! library to access EVE data and process it for its needs.
//!
//! ## Feature flags
//!
//! This library provides two different data handlers, with their availability controlled via
//! [feature flags] to reduce the amount of compiled code.
//!
//! - `full`: Enables both handlers.
//! - `http`: Enables handler which fetches data over HTTP.
//! - `file`: Enables handler which reads data from local files.

#[cfg(feature = "file")]
pub use handler_file::PhbFileEdh;
#[cfg(feature = "http")]
pub use handler_http::PhbHttpEdh;
pub use util::{Error, ErrorKind, Result};

mod data;
mod fsd;
#[cfg(feature = "file")]
mod handler_file;
#[cfg(feature = "http")]
mod handler_http;
mod util;
