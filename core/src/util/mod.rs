//! Various helper entities used throughout the library.

pub use err_res::public::{Error, ErrorKind, Result};
pub(crate) use err_res::{
    debug::{DebugError, DebugResult},
    internal::{IntError, IntResult},
};
pub(crate) use funcs::vec_push_opt;
pub(crate) use storage::{extend_vec_from_l1set, KsL1Set, KsL2Set};
pub(crate) use traits::Named;

mod err_res;
mod funcs;
mod storage;
mod traits;
