use crate::{
    num::Value,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(crate) trait LimitAmount {
    fn limit_amount(&mut self, limit: Value);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + LimitAmount,
{
    pub(in crate::svc::vast::aggr) fn limit_amount(&mut self, limit: Value) {
        match self {
            Self::Simple(inner) => inner.limit_amount(limit),
            Self::Complex(inner) => inner.limit_amount(limit),
        }
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: Value) {
        self.amount.limit_amount(limit);
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: Value) {
        self.amount.limit_amount(limit);
    }
}
