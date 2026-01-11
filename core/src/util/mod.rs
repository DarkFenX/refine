//! Various helper entities used throughout the library.

pub(crate) use float::{
    FLOAT_TOLERANCE, ceil_f64_to_u32, ceil_f64_to_usize, ceil_tick, ceil_unerr, floor_tick, floor_unerr, round,
    round_f64_to_i32, round_f64_to_u32, sig_round, trunc_f64_to_u32,
};
pub(crate) use func::vec_push_opt;
pub(crate) use storage::{CMap, RMap, RMapRMap, RMapRMapRMap, RMapRSet, RMapVec, RSet, extend_vec_from_map_set_l1};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::{LibConvertExtend, LibDefault, LibGetId, LibIncrement, LibMax, LibNamed};

mod float;
mod func;
mod storage;
mod str_err;
mod traits;
