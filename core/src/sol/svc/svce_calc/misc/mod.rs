pub(in crate::sol::svc::svce_calc) use attr_spec::SolAttrSpec;
pub use attr_val::SolAttrVal;
pub(in crate::sol::svc::svce_calc) use attr_val_data::SolAttrValData;
pub(in crate::sol::svc::svce_calc) use loc_kind::SolLocationKind;
pub(in crate::sol::svc::svce_calc) use modification::SolModification;
pub(in crate::sol::svc::svce_calc) use modification_key::SolModificationKey;
pub(in crate::sol::svc::svce_calc) use resist::get_proj_effect_resist_attr_id;

mod attr_spec;
mod attr_val;
mod attr_val_data;
mod item_exts;
mod loc_kind;
mod modification;
mod modification_key;
mod resist;
