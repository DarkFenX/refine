use crate::{api::AttrVals, def::AttrVal};

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    pub(crate) base: AttrVal,
    pub(crate) dogma: AttrVal,
    pub(crate) extra: AttrVal,
}
impl From<CalcAttrVals> for AttrVals {
    fn from(calc_vals: CalcAttrVals) -> Self {
        Self {
            base: calc_vals.base,
            modified: calc_vals.extra,
        }
    }
}
