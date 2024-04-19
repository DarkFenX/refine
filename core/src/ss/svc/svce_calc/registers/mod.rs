pub(in crate::ss::svc::svce_calc) use buff::SsBuffRegister;
pub(in crate::ss::svc::svce_calc) use dependency::{SsAttrSpec, SsDependencyRegister};
pub(in crate::ss::svc::svce_calc) use modifier::{SsFleetUpdates, SsModifierRegister};
pub(in crate::ss::svc::svce_calc) use revision::SsRevisionRegister;
pub(in crate::ss::svc::svce_calc) use target::SsTargetRegister;

mod buff;
mod dependency;
mod modifier;
mod revision;
mod target;
