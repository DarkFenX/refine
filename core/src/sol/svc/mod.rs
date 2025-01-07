//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::sol::svc) use misc::debug;
pub use misc::SolEffectInfo;
pub(in crate::sol) use svc::SolSvc;

pub(crate) mod calc;
mod misc;
mod rest;
mod svc;
mod svce_debug;
mod svce_effect;
mod svce_interface;
mod svce_notify;
