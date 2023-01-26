//! Cache handling interface.
//!
//! Cache handlers allow the library to store [cacheable types](crate::ct) persistently and load
//! them on subsequent runs, avoiding need to generate them on every run.

pub use data::CHData;
pub use handler::CacheHandler;

mod data;
mod handler;
