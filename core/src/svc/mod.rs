//! Solar system services - attribute & stats calculations, validation, and so on.

pub(crate) use misc::SvcCtx;
pub(crate) use svc::Svc;

pub(crate) mod calc;
pub(crate) mod efuncs;
pub(crate) mod eprojs;
pub(crate) mod err;
mod misc;
mod svc;
mod svce_debug;
mod svce_notify;
mod svce_request;
pub(crate) mod vast;
