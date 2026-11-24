use crate::{UnitInterval, Value, ad::AMutaAttrRange};

#[derive(Copy, Clone)]
pub(crate) struct RMutaAttrRange {
    /// For some attr mutations, min mult has higher value than max. Some parts of the lib rely on
    /// this data as-is, but some need actual min and actual max, so we're storing both.
    pub(crate) mult_min_raw: Value,
    pub(crate) mult_max_raw: Value,
    pub(crate) mult_min_math: Value,
    pub(crate) mult_max_math: Value,
}
impl RMutaAttrRange {
    /// Roll which turns unmutated value into absolute one, clamped when the latter is out of range.
    ///
    /// Returns None if mutation range limits the attribute to a single value.
    pub(crate) fn resolve_absolute_into_roll(
        &self,
        unmutated_value: Value,
        absolute_value: Value,
    ) -> Option<UnitInterval> {
        let min_value = unmutated_value * self.mult_min_raw;
        let max_value = unmutated_value * self.mult_max_raw;
        if min_value == max_value {
            return None;
        }
        let value = (absolute_value - min_value) / (max_value - min_value);
        Some(UnitInterval::from_value_clamped(value))
    }
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
