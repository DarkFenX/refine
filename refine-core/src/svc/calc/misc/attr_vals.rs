use crate::num::Value;

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    pub(crate) base: Value,
    pub(in crate::svc::calc) dogma: Value,
    pub(crate) extra: Value,
}
