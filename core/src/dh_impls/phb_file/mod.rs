//! Data handler implementation which uses locally stored JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source.

pub use handler::PhbFileDHandler;

mod address;
mod error;
mod data;
mod fsd;
mod handler;
