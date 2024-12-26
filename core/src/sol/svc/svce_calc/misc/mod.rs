pub(in crate::sol::svc::svce_calc) use attr_spec::SolAttrSpec;
pub use attr_val::SolAttrVal;
pub(in crate::sol::svc::svce_calc) use attr_val_data::{SolAttrValData, SolItemAttrValData};
pub(in crate::sol::svc::svce_calc) use loc_kind::SolLocationKind;
pub(in crate::sol::svc::svce_calc) use modification::SolModification;
pub(in crate::sol::svc::svce_calc) use modification_key::SolModificationKey;

mod attr_spec;
mod attr_val;
mod attr_val_data;
mod item_exts;
mod loc_kind;
mod modification;
mod modification_key;
mod rah;
