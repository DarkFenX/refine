use super::limit_amount::LimitAmount;
use crate::{
    def::AttrVal,
    svc::output::{Output, OutputComplex, OutputSimple},
};

impl<T> Output<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc::aggr) fn amount_sum(&self) -> T {
        match self {
            Self::Simple(inner) => inner.amount_sum(),
            Self::Complex(inner) => inner.amount_sum(),
        }
    }
}
impl<T> Output<T>
where
    T: Copy + LimitAmount,
{
    pub(in crate::svc::aggr) fn limit_amount(&mut self, limit: AttrVal) {
        match self {
            Self::Simple(inner) => inner.limit_amount(limit),
            Self::Complex(inner) => inner.limit_amount(limit),
        }
    }
}

impl<T> OutputSimple<T>
where
    T: Copy,
{
    fn amount_sum(&self) -> T {
        self.amount
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: AttrVal) {
        self.amount.limit_amount(limit);
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    fn amount_sum(&self) -> T {
        self.amount * AttrVal::from(self.repeats)
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: AttrVal) {
        self.amount.limit_amount(limit);
    }
}
