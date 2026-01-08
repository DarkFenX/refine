use crate::{misc::Value, svc::calc::CalcAttrVals};

#[derive(Copy, Clone)]
pub struct AttrVals {
    pub base: Value,
    pub modified: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AttrVals {
    pub(in crate::api) fn from_calc_attr_vals(calc_attr_vals: CalcAttrVals) -> Self {
        AttrVals {
            base: calc_attr_vals.base,
            modified: calc_attr_vals.extra,
        }
    }
}
