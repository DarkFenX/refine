//! Various helper entities used throughout the library.

pub(crate) use funcs::{round, sig_round, vec_push_opt};
pub use storage::{HMap, HSet, Map};
pub(crate) use storage::{HMapHMap, HMapHSet, HMapVec, MapMap, MapSet, MapVec, Set, extend_vec_from_map_set_l1};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::Named;

mod funcs;
mod storage;
mod str_err;
mod traits;
