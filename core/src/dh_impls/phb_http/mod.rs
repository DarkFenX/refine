//! Data handler implementation which uses HTTP-served JSON produced by
//! [Phobos](https://github.com/pyfa-org/Phobos) as a data source.

pub use handler::PhbHttpDHandler;

mod data;
mod error;
mod fsd;
mod handler;
