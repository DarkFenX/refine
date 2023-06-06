//! EVE data handlers which use JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source. They enable `reefast-core`
//! library to access EVE data and process it for its needs.
//!
//! ## Feature flags
//!
//! This library provides two different data handlers, with their availability controlled via
//! feature flags to reduce the amount of compiled code.
//!
//! - `full`: Enables both handlers.
//! - `phb-http`: Enables handler which fetches data over HTTP.
//! - `phb-file`: Enables handler which reads data from local files.

#[cfg(feature = "phb-file")]
pub use phb::PhbFileEdh;
#[cfg(feature = "phb-http")]
pub use phb::PhbHttpEdh;
pub use util::{Error, ErrorKind, Result};

mod phb;
mod util;
