//! JSON cache storage implementation.
//!
//! This cache handler implements persistent cache store in the form of zstd-compressed JSON. When
//! data is loaded, cacheable types are stored in RAM, thus it provides extremely fast access, but
//! has noticeable initialization time and RAM consumption.

pub use handler::JsonFileCHandler;

mod data;
mod handler;
