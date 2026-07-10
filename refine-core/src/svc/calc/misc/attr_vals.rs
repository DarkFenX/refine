use crate::num::Value;

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    pub(crate) base: Value,
    pub(crate) dogma: Value,
    pub(crate) extra: Value,
}
