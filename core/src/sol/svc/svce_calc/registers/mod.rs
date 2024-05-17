pub(in crate::sol::svc::svce_calc) use affectee::SolAffecteeRegister;
pub(in crate::sol::svc::svce_calc) use buff::SolBuffRegister;
pub(in crate::sol::svc::svce_calc) use dependency::{SolAttrSpec, SolDependencyRegister};
pub(in crate::sol::svc::svce_calc) use modifier::{SolFleetUpdates, SolModifierRegister};
pub(in crate::sol::svc::svce_calc) use projection::SolProjectionRegister;
pub(in crate::sol::svc::svce_calc) use revision::SolRevisionRegister;

mod affectee;
mod buff;
mod dependency;
mod modifier;
mod projection;
mod revision;
