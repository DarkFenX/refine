use crate::{Value, svc::calc::CalcAttrVals};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemAttrValues {
    pub base: Value,
    pub modified: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAttrValues {
    pub(in crate::api) fn from_calc_attr_vals(calc_attr_vals: CalcAttrVals) -> Self {
        ItemAttrValues {
            base: calc_attr_vals.base,
            modified: calc_attr_vals.extra,
        }
    }
}
