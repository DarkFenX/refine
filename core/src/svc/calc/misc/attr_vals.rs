use crate::{api::AttrVals, misc::Value};

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    pub(crate) base: Value,
    pub(crate) dogma: Value,
    pub(crate) extra: Value,
}
impl From<CalcAttrVals> for AttrVals {
    fn from(calc_vals: CalcAttrVals) -> Self {
        Self {
            base: calc_vals.base,
            modified: calc_vals.extra,
        }
    }
}
