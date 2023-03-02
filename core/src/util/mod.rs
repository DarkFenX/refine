//! Various helper entities used throughout the library.

pub(crate) use funcs::vec_push_opt;
pub(crate) use keyed_storage::KeyedStorage;
pub(crate) use traits::Named;

pub(crate) mod err_res;
mod funcs;
mod keyed_storage;
mod traits;
