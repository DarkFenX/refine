//! Cache implementation without persistence.
//!
//! This cache handler stores data only in RAM. Access to data is fast, but has noticeable RAM
//! consumption and cache has to be rebuilt every time.

pub use handler::RamOnlyCHandler;

mod handler;
