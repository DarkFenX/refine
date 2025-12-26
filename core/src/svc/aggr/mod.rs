//! Aggregators convert cycle sequence and output-per-cycle into a more processed form like
//! single-number stats.

pub(in crate::svc) use local_first::aggr_local_first_per_second;
pub(in crate::svc) use local_looped::aggr_local_looped_per_second;

mod local_first;
mod local_inv_data;
mod local_looped;
mod proj_inv_data;
mod traits;
