//! Various helper entities used throughout the library.

pub(crate) use float::{
    FLOAT_TOLERANCE, ceil_f64_to_u32, ceil_tick, ceil_unerr, floor_tick, floor_unerr, round, round_f64_to_i32,
    round_f64_to_u32, sig_round, sum_pai_owned, sum_pai_ref, trunc_f64_to_u32,
};
pub(crate) use func::vec_push_opt;
pub(crate) use iter_peek_prefetch::PrefetchPeekable;
pub(crate) use state::{State3, State4, State5};
pub(crate) use storage::{
    CMap, PSlab, RMap, RMapRMap, RMapRMapRMap, RMapRSet, RMapVec, ROrdSet, RSet, SSLabRSet, SSlab, SSlabUnchecked,
    SlabId,
};
pub(crate) use traits::{LibConverter, LibDefault, LibGetId, LibIncrement, LibMax, LibNamed};

mod float;
mod func;
mod iter_peek_prefetch;
mod state;
mod storage;
mod traits;
