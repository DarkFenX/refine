//! Various helper entities used throughout the library.

pub use adj_count::AdjustableCount;
pub(crate) use funcs::{round, sig_round, vec_push_opt};
pub use storage::{StMap, StSet};
pub(crate) use storage::{StMapMap, StMapSetL1, StMapSetL2, StMapVecL1, extend_vec_from_map_set_l1};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::Named;

mod adj_count;
mod funcs;
mod storage;
mod str_err;
mod traits;
