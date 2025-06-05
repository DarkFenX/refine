//! Solar system services - attribute & stats calculations, validation, and so on.

use misc::{AttrSpec, EffectSpec, get_resist_a_attr_id, get_resist_mult_val};
pub(in crate::sol) use svc::Svc;

pub(crate) mod calc;
mod misc;
mod svc;
mod svce_debug;
mod svce_notify;
pub(crate) mod vast;
