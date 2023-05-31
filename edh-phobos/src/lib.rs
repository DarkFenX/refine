//! EVE data handlers which use JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source.

#[cfg(feature = "file")]
pub use handler_file::PhbFileDHandler;
#[cfg(feature = "http")]
pub use handler_http::PhbHttpDHandler;
pub use util::{Error, ErrorKind, Result};

mod data;
mod fsd;
#[cfg(feature = "file")]
mod handler_file;
#[cfg(feature = "http")]
mod handler_http;
mod util;
