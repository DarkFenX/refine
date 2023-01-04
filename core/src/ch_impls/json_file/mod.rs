//! JSON cache storage implementation.
//!
//! This cache handler implements persistent cache store in the form of compressed JSON. When data
//! is loaded, cacheable types are stored in memory, thus it provides extremely fast access, but has
//! subpar initialization time and memory consumption.

pub use handler::JsonFileCHandler;


mod handler;
