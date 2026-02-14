//! Aggregators convert cycle sequence and output-per-cycle into a more processed form like
//! single-number stats.

pub(in crate::svc::vast) use accum::{SeqAccum, SeqInstanceAccum};
pub(in crate::svc::vast) use local_clip::aggr_local_clip;
pub(in crate::svc::vast) use local_first::aggr_local_first;
pub(in crate::svc::vast) use local_iter::aggr_local_iter;
pub(in crate::svc::vast) use local_looped::aggr_local_looped;
pub(in crate::svc::vast) use local_shared::{AggrLocalInvData, get_local_output};
pub(in crate::svc::vast) use local_time::aggr_local_time;
pub(in crate::svc::vast) use proj_clip::aggr_proj_clip;
pub(in crate::svc::vast) use proj_first::aggr_proj_first;
pub(in crate::svc::vast) use proj_looped::aggr_proj_looped;
pub(in crate::svc::vast) use proj_shared::{AggrProjInvData, get_proj_output};
pub(in crate::svc::vast) use proj_time::aggr_proj_time;

mod accum;
mod local_clip;
mod local_first;
mod local_iter;
mod local_looped;
mod local_shared;
mod local_time;
mod proj_clip;
mod proj_first;
mod proj_iter;
mod proj_looped;
mod proj_shared;
mod proj_time;
mod shared;
mod shared_iter;
mod shared_time;
mod traits;
