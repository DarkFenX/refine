//! Solar system services - attribute & stats calculations, validation, and so on.

pub(crate) use ctx::SvcCtx;
pub(crate) use svc::Svc;

pub(crate) mod calc;
mod ctx;
pub(crate) mod cycle;
pub(crate) mod eff_funcs;
pub(crate) mod eff_projs;
pub(crate) mod err;
pub(crate) mod output;
mod svc;
mod svce_debug;
mod svce_notify;
mod svce_request;
pub(crate) mod vast;
