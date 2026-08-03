use crate::{AttrId, Value, svc::calc::CalcAttrVals};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemAttrInfo {
    pub id: AttrId,
    pub base_value: Value,
    pub modified_value: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAttrInfo {
    pub(in crate::api) fn from_calc_attr_vals(id: AttrId, calc_attr_vals: CalcAttrVals) -> Self {
        ItemAttrInfo {
            id,
            base_value: calc_attr_vals.base,
            modified_value: calc_attr_vals.extra,
        }
    }
}
