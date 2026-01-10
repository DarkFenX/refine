use crate::{ad::AMutaAttrRange, num::Value};

#[derive(Copy, Clone)]
pub(crate) struct RMutaAttrRange {
    pub(crate) min_mult: Value,
    pub(crate) max_mult: Value,
}
impl RMutaAttrRange {
    pub(in crate::rd) fn from_a_attr_range(a_attr_range: &AMutaAttrRange) -> Self {
        Self {
            min_mult: Value::from_a_value(a_attr_range.min_mult),
            max_mult: Value::from_a_value(a_attr_range.max_mult),
        }
    }
}
