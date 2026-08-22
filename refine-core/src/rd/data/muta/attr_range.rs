use crate::{ad::AMutaAttrRange, num::Value};

#[derive(Copy, Clone)]
pub(crate) struct RMutaAttrRange {
    // For some attr mutations, min mult has higher value than max. Some parts of the lib rely on
    // this data as-is, but some need actual min and actual max, so we're storing both.
    pub(crate) mult_min_raw: Value,
    pub(crate) mult_max_raw: Value,
    pub(crate) mult_min_math: Value,
    pub(crate) mult_max_math: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RMutaAttrRange {
    pub(super) fn from_a_attr_range(a_attr_range: &AMutaAttrRange) -> Self {
        let mult_min_raw = Value::from_a_value(a_attr_range.mult_min);
        let mult_max_raw = Value::from_a_value(a_attr_range.mult_max);
        Self {
            mult_min_raw,
            mult_max_raw,
            mult_min_math: Value::min(mult_min_raw, mult_max_raw),
            mult_max_math: Value::max(mult_min_raw, mult_max_raw),
        }
    }
}
