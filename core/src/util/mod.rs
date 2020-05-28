pub use error::Error;
pub(crate) use funcs::vec_push_opt;
pub use result::Result;
pub(crate) use traits::Named;

mod error;
mod funcs;
mod result;
mod traits;
