//! Effect-related functions.

pub(crate) use duration::{get_effect_duration_s, get_espec_duration_s};
pub(in crate::svc) use resist::get_resist_attr_key;
pub(crate) use resist::{get_effect_resist_mult, get_resist_mult_by_projectee_aspec};

mod duration;
mod resist;
