pub(in crate::sol::svc::svce_calc) use aggr_mode::SolModAggrMode;
pub(in crate::sol::svc::svce_calc) use custom::extend_with_custom_mods;
pub(in crate::sol::svc::svce_calc) use domain::SolModDomain;
pub(in crate::sol::svc::svce_calc) use mod_type::SolModType;
pub(in crate::sol::svc::svce_calc) use modifier::SolAttrMod;
pub(in crate::sol::svc::svce_calc) use op::SolModOp;
use src::SolAttrModSrc;
pub(in crate::sol::svc::svce_calc) use tgt_filter::SolAffecteeFilter;

mod aggr_mode;
mod custom;
pub(in crate::sol::svc::svce_calc) mod debug;
mod domain;
mod mod_type;
mod modifier;
mod op;
mod src;
mod tgt_filter;
