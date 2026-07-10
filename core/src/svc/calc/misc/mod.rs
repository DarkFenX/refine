pub(in crate::svc::calc) use attr_val_data::{AttrValData, ItemAttrData, ItemAttrPostproc};
pub(crate) use attr_vals::CalcAttrVals;
pub(crate) use info::{CalcModInfo, CalcModInfoAffector};
pub(in crate::svc::calc) use loc_kind::LocationKind;
pub(in crate::svc::calc) use modification::CalcModification;
pub(in crate::svc::calc) use modification_key::CalcModificationKey;

mod attr_val_data;
mod attr_vals;
mod info;
mod loc_kind;
mod modification;
mod modification_key;
mod u_item_exts;
