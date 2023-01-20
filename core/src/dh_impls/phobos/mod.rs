//! Data handlers which use JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source.

pub use handler_file::PhbFileDHandler;
pub use handler_http::PhbHttpDHandler;

mod data;
mod fsd;
mod handler_file;
mod handler_http;
