use crate::{ad::AMutaAttrRange, misc::Value};

#[derive(Copy, Clone)]
pub(crate) struct RMutaAttrRange {
    pub(crate) min_mult: Value,
    pub(crate) max_mult: Value,
}
impl RMutaAttrRange {
    pub(in crate::rd) fn from_a_attr_range(a_attr_range: &AMutaAttrRange) -> Self {
        Self {
            min_mult: a_attr_range.min_mult.into(),
            max_mult: a_attr_range.max_mult.into(),
        }
    }
}
