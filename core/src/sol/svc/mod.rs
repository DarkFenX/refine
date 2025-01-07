//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::sol) use svc::SolSvc;

pub(crate) mod calc;
mod debug;
mod rest;
mod running_effects;
mod svc;
mod svce_debug;
mod svce_effect;
mod svce_interface;
mod svce_notify;
