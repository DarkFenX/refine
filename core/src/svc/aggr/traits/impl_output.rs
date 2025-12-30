use super::limit_amount::LimitAmount;
use crate::{
    def::{AttrVal, OF},
    svc::output::{Output, OutputComplex, OutputSimple},
    util::trunc_unerr,
};

impl<T> Output<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc::aggr) fn get_amount_sum(&self) -> T {
        match self {
            Self::Simple(inner) => inner.get_amount_sum(),
            Self::Complex(inner) => inner.get_amount_sum(),
        }
    }
}
impl<T> Output<T>
where
    T: Copy + Default + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc::aggr) fn get_amount_sum_by_time(&self, time: AttrVal) -> T {
        match self {
            Output::Simple(inner) => inner.get_amount_sum_by_time(time),
            Output::Complex(inner) => inner.get_amount_sum_by_time(time),
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
    fn get_amount_sum(&self) -> T {
        self.amount
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + Default,
{
    fn get_amount_sum_by_time(&self, time: AttrVal) -> T {
        match self.delay <= time {
            true => self.amount,
            false => T::default(),
        }
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
    fn get_amount_sum(&self) -> T {
        self.amount * AttrVal::from(self.repeats)
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + Default + std::ops::Mul<AttrVal, Output = T>,
{
    fn get_amount_sum_by_time(&self, time: AttrVal) -> T {
        let after_delay = time - self.delay;
        if after_delay < OF(0.0) {
            return T::default();
        }
        let count = after_delay / self.interval;
        if !count.is_finite() {
            return T::default();
        }
        let count = trunc_unerr(count);
        if count < OF(0.0) {
            return T::default();
        }
        self.amount * count
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
