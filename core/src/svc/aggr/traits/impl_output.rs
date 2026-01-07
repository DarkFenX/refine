use super::limit_amount::LimitAmount;
use crate::{
    misc::{PValue, Value},
    svc::output::{Output, OutputComplex, OutputSimple},
};

impl<T> Output<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
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
    T: Copy + Default + std::ops::Mul<PValue, Output = T>,
{
    pub(in crate::svc::aggr) fn get_amount_sum_by_time(&self, time: PValue) -> T {
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
    pub(in crate::svc::aggr) fn limit_amount(&mut self, limit: Value) {
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
    fn get_amount_sum_by_time(&self, time: PValue) -> T {
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
    fn limit_amount(&mut self, limit: Value) {
        self.amount.limit_amount(limit);
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    fn get_amount_sum(&self) -> T {
        self.amount * self.repeats.into_pvalue()
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + Default + std::ops::Mul<PValue, Output = T>,
{
    fn get_amount_sum_by_time(&self, time: PValue) -> T {
        let after_delay = time - self.delay;
        let after_delay = match after_delay >= Value::ZERO {
            true => PValue::from_val_unchecked(after_delay),
            false => return T::default(),
        };
        let count = after_delay / self.interval;
        if !count.is_finite() {
            return T::default();
        }
        self.amount * count.floor_unerr()
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
