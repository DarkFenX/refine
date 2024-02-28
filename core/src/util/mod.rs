//! Various helper entities used throughout the library.

pub(crate) use err_res::internal::{IntError, IntResult};
pub use err_res::public::{Error, ErrorKind, Result};
pub(crate) use funcs::vec_push_opt;
pub(crate) use keyed_storage::{extend_vec_from_storage, KeyedStorage1L, KeyedStorage2L};
pub(crate) use traits::Named;

mod err_res;
mod funcs;
mod keyed_storage;
mod traits;
