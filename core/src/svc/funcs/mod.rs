//! Helper functions.

pub(in crate::svc) use attr_flag::{is_attr_flag_set, is_oattr_flag_set};
pub(crate) use effect_duration::get_effect_duration_s;
pub(in crate::svc) use effect_duration::get_espec_duration_s;
pub(crate) use item_mobility::{get_sig_radius, get_speed};

mod attr_flag;
mod effect_duration;
mod item_mobility;
