//! Solar system services - attribute & stats calculations, validation, and so on.

pub(crate) use misc::{SvcCtx, get_proj_mult, get_resist_mult};
use misc::{get_resist_a_attr_id, get_resist_mult_val_by_projectee_aspec};
pub(crate) use svc::Svc;

pub(crate) mod calc;
pub(crate) mod eprojs;
pub(crate) mod err;
mod misc;
mod svc;
mod svce_debug;
mod svce_notify;
mod svce_request;
pub(crate) mod vast;
