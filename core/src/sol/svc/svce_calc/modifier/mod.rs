use affector_val::SolAffectorValue;
pub(in crate::sol::svc::svce_calc) use aggr_mode::SolAggrMode;
pub(in crate::sol::svc::svce_calc) use custom::extend_with_custom_mods;
pub(in crate::sol::svc::svce_calc) use domain::SolDomain;
pub(in crate::sol::svc::svce_calc) use kind::SolModifierKind;
pub(in crate::sol::svc::svce_calc) use modifier::SolModifier;
pub(in crate::sol::svc::svce_calc) use op::SolOp;
pub(in crate::sol::svc::svce_calc) use tgt_filter::SolAffecteeFilter;

mod affector_val;
mod aggr_mode;
mod custom;
pub(in crate::sol::svc::svce_calc) mod debug;
mod domain;
mod kind;
mod modifier;
mod op;
mod tgt_filter;
