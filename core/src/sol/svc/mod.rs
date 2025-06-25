//! Solar system services - attribute & stats calculations, validation, and so on.

pub(in crate::sol::svc) use misc::SvcCtx;
use misc::{AttrSpec, EffectSpec, get_resist_a_attr_id, get_resist_mult_val, get_resist_mult_val_by_projectee_aspec};
pub(in crate::sol) use svc::Svc;

pub(crate) mod calc;
pub(crate) mod eprojs;
mod misc;
mod svc;
mod svce_debug;
mod svce_notify;
mod svce_request;
pub(crate) mod vast;
