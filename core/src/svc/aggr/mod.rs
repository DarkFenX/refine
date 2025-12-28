//! Aggregators convert cycle sequence and output-per-cycle into a more processed form like
//! single-number stats.

pub(in crate::svc) use local_clip::aggr_local_clip_amount;
pub(in crate::svc) use local_first::{aggr_local_first_amount, aggr_local_first_output, aggr_local_first_ps};
pub(in crate::svc) use local_looped::{aggr_local_looped_amount, aggr_local_looped_ps};
pub(in crate::svc) use proj_clip::aggr_proj_clip_amount;
pub(in crate::svc) use proj_first::{
    aggr_proj_first_amount, aggr_proj_first_max, aggr_proj_first_output, aggr_proj_first_ps,
};
pub(in crate::svc) use proj_looped::{aggr_proj_looped_amount, aggr_proj_looped_max, aggr_proj_looped_ps};

mod local_clip;
mod local_first;
mod local_inv_data;
mod local_looped;
mod proj_clip;
mod proj_first;
mod proj_inv_data;
mod proj_looped;
mod shared;
mod traits;
