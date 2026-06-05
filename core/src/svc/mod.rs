//! Solar system services - attribute & stats calculations, validation, and so on.

pub(crate) use ctx::SvcCtx;
pub(crate) use svc::Svc;

mod api;
pub(crate) mod calc;
mod ctx;
pub(crate) mod cycle;
pub(crate) mod eff_projs;
pub(crate) mod err;
pub(crate) mod funcs;
pub(crate) mod output;
mod spool;
mod svc;
mod svce_debug;
mod svce_notify;
pub(crate) mod vast;
