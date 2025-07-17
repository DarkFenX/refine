//! Effect-related functions.

pub(in crate::svc) use duration::get_effect_duration_s;
pub(crate) use duration::get_espec_duration_s;
pub(crate) use projection::get_proj_mult;
pub(crate) use resist::get_resist_mult;
pub(in crate::svc) use resist::{get_resist_a_attr_id, get_resist_mult_val_by_projectee_aspec};

mod duration;
mod projection;
mod resist;
