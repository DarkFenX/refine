//! Aggregators convert cycle info and output-per-cycle info into more ready-to-consume form like
//! single-number stats.

pub(in crate::svc) use local_first::aggr_local_first_per_second;
pub(in crate::svc) use local_looped::aggr_local_looped_per_second;

mod local_first;
mod local_inv_data;
mod local_looped;
mod shared;
mod traits;
