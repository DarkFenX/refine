//! Helper functions.

pub(crate) use effect_duration::{get_effect_duration_s, get_espec_duration_s};
pub(in crate::svc) use effect_resist::get_resist_attr_rid;
pub(crate) use effect_resist::{get_effect_resist_mult, get_resist_mult_by_projectee_aspec};
pub(crate) use item_mobility::{get_sig_radius, get_speed};

mod effect_duration;
mod effect_resist;
mod item_mobility;
