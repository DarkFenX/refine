//! Solar system services - attribute & stats calculations, validation, and so on.

use attr_spec::AttrSpec;
pub(crate) use calc::Calc;
pub(crate) use ctx::SvcCtx;
use effect_spec::EffectSpec;
pub use spool::{ItemSpoolInfo, SpoolCycleInfo};
pub(crate) use svc::Svc;
use vast::Vast;

mod api;
mod attr_spec;
pub(crate) mod calc;
mod ctx;
pub(crate) mod cycle;
mod eff_projs;
mod effect_spec;
pub(crate) mod err;
pub(crate) mod funcs;
pub(crate) mod output;
mod r_resist_exts;
mod spool;
mod svc;
mod svce_notify;
pub(crate) mod vast;
