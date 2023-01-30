//! Cache handling interface.
//!
//! Cache handlers allow the library to store [cacheable types](crate::ct) persistently and load
//! them on subsequent runs, avoiding need to generate them on every run.

pub use data::ChData;
pub use handler::CacheHandler;
pub use result::Result;

mod data;
mod handler;
mod result;
