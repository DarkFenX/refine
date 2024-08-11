//! Various helper entities used throughout the library.

pub(crate) use funcs::vec_push_opt;
pub use storage::StMap;
pub(crate) use storage::{extend_vec_from_map_set_l1, StMapSetL1, StMapSetL2, StMapVecL1, StSet};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::Named;

mod funcs;
mod storage;
mod str_err;
mod traits;
