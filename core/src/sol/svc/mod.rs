//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::sol::svc) use misc::debug;
pub(in crate::sol) use svc::SolSvcs;
pub use svce_calc::{SolAffectorInfo, SolModificationInfo, SolOpInfo};

pub(in crate::sol) mod err;
mod misc;
mod svc;
pub(crate) mod svce_calc;
mod svce_debug;
mod svce_effect;
mod svce_interface;
mod svce_notify;
