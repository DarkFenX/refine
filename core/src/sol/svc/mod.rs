//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::sol::svc) use misc::debug;
pub(in crate::sol) use svc::SolSvcs;
pub use svce_calc::{SolAttrVal, SolModInfo, SolModOpInfo, SolModSrcInfo, SolModSrcValInfo};

mod misc;
mod svc;
mod svce_calc;
mod svce_debug;
mod svce_routing;
