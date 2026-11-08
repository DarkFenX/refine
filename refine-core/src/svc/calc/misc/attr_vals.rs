use crate::{Value, num::ValueNiched};

#[derive(Copy, Clone)]
pub(crate) struct CalcAttrVals {
    // Rarely used, makes space for all the types wrapping this one to get wrapped into None
    pub(crate) base: ValueNiched,
    pub(in crate::svc::calc) dogma: Value,
    pub(crate) extra: Value,
}
impl CalcAttrVals {
    pub(in crate::svc::calc) fn get_value(&self, kind: CalcValueKind) -> Value {
        match kind {
            CalcValueKind::Dogma => self.dogma,
            CalcValueKind::Extra => self.extra,
        }
    }
}

#[derive(std::marker::ConstParamTy, Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::calc) enum CalcValueKind {
    Dogma,
    Extra,
}
