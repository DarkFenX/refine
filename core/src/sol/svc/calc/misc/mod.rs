pub(in crate::sol::svc::calc) use accum_mgr::AccumMgr;
pub(in crate::sol::svc::calc) use attr_spec::AttrSpec;
pub use attr_val::CalcAttrVal;
pub(in crate::sol::svc::calc) use attr_val_data::{
    AttrValData, FTR_COUNT_ATTR, ItemAttrPostprocs, ItemAttrValData, SKILL_LVL_ATTR,
};
pub(in crate::sol::svc::calc) use loc_kind::LocationKind;
pub(in crate::sol::svc::calc) use modification::Modification;
pub(in crate::sol::svc::calc) use modification_key::ModificationKey;

mod accum_mgr;
mod attr_spec;
mod attr_val;
mod attr_val_data;
mod item_exts;
mod loc_kind;
mod modification;
mod modification_key;
