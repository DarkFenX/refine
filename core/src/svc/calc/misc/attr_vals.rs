use crate::{api::AttrVals, misc::Value};

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    pub(crate) base: Value,
    pub(crate) dogma: Value,
    pub(crate) extra: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CalcAttrVals {
    pub(in crate::svc::calc) fn into_attr_vals(self) -> AttrVals {
        AttrVals {
            base: self.base,
            modified: self.extra,
        }
    }
}
