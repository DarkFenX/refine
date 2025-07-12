//! Effect-related functions.

pub(in crate::svc) use cycle_count::get_espec_cycle_count;
pub(crate) use cycle_time::get_espec_cycle_time;
pub(in crate::svc) use cycle_time::{get_effect_cycle_time, has_cycle_time};

mod cycle_count;
mod cycle_time;
