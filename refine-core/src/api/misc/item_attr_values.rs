use crate::{Value, svc::calc::CalcAttrVals};

/// Values of an item attribute.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemAttrValues {
    /// Value before modifications are applied onto it.
    ///
    /// On mutated items, base value includes modifications coming from mutations.
    pub base: Value,
    /// Value with all the modifications applied.
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
