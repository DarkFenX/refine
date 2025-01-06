//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::sol::svc) use misc::debug;
pub use misc::SolEffectInfo;
pub(in crate::sol) use svc::SolSvcs;

pub(in crate::sol) mod err;
mod misc;
mod svc;
pub(crate) mod svce_calc;
mod svce_debug;
mod svce_effect;
mod svce_effect_attrs;
mod svce_interface;
mod svce_notify;
mod svce_stats;
