//! Effect-related functions.

pub(in crate::svc) use cycle_count::get_espec_cycle_count;
pub(in crate::svc) use duration::get_effect_duration_s;
pub(crate) use duration::get_espec_duration_s;

mod cycle_count;
mod duration;
