pub use attr_val::CalcAttrVal;
pub(in crate::svc::calc) use attr_val_data::{AttrValData, ItemAttrData, ItemAttrPostprocs};
pub(in crate::svc::calc) use loc_kind::LocationKind;
pub(in crate::svc::calc) use modification::CalcModification;
pub(in crate::svc::calc) use modification_key::CalcModificationKey;

mod attr_val;
mod attr_val_data;
mod loc_kind;
mod modification;
mod modification_key;
mod u_item_exts;
