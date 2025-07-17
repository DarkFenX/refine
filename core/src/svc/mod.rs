//! Solar system services - attribute & stats calculations, validation, and so on.

pub(crate) use ctx::SvcCtx;
pub(crate) use svc::Svc;

pub(crate) mod calc;
mod ctx;
pub(crate) mod cycle;
pub(crate) mod efuncs;
pub(crate) mod eprojs;
pub(crate) mod err;
mod svc;
mod svce_debug;
mod svce_notify;
mod svce_request;
pub(crate) mod vast;
