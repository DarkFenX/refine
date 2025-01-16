//! Solar system services - attribute & stats calculations, validation, and so on.

pub(in crate::sol) use svc::SolSvc;

pub(crate) mod calc;
mod running_effects;
mod svc;
mod svce_debug;
mod svce_effect;
mod svce_interface;
mod svce_notify;
pub(crate) mod vast;
