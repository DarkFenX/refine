use ordered_float::Float;

use super::shared::OutputIterItem;
use crate::{
    misc::{Count, PValue, Value},
    util::FLOAT_TOLERANCE,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<T>
where
    T: Copy,
{
    pub(crate) amount: T,
    pub(crate) delay: PValue,
    // Total count of times amount is output
    pub(crate) repeats: Count,
    pub(crate) interval: PValue,
}
impl<T> OutputComplex<T>
where
    T: Copy,
{
    pub(super) fn get_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_max_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_completion_time(&self) -> PValue {
        if self.repeats < Count::ONE {
            return PValue::ZERO;
        };
        self.delay + self.interval * PValue::from_f64_unchecked((self.repeats.into_u32() - 1) as f64)
    }
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputComplexAmountIter::new(self)
    }
}
impl OutputComplex<Value> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount.abs() > PValue::FLOAT_TOLERANCE
    }
    pub(super) fn absolute_impact(&self) -> PValue {
        self.amount.abs() * PValue::from_f64_unchecked(self.repeats.into_u32() as f64)
    }
    pub(super) fn add_amount(&mut self, amount: AttrVal) {
        self.amount += amount;
    }
}
impl<T> std::ops::Neg for OutputComplex<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.amount = -self.amount;
        self
    }
}
impl<T> std::ops::Mul<AttrVal> for OutputComplex<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            delay: self.delay,
            repeats: self.repeats,
            interval: self.interval,
        }
    }
}
impl<T> std::ops::MulAssign<AttrVal> for OutputComplex<T>
where
    T: Copy + std::ops::MulAssign<AttrVal>,
{
    fn mul_assign(&mut self, rhs: AttrVal) {
        self.amount.mul_assign(rhs);
    }
}

struct OutputComplexAmountIter<'a, T>
where
    T: Copy,
{
    output: &'a OutputComplex<T>,
    cycles_done: DefCount,
}
impl<'a, T> OutputComplexAmountIter<'a, T>
where
    T: Copy,
{
    fn new(output: &'a OutputComplex<T>) -> Self {
        Self { output, cycles_done: 0 }
    }
}
impl<T> Iterator for OutputComplexAmountIter<'_, T>
where
    T: Copy,
{
    type Item = OutputIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.output.repeats {
            return None;
        }
        self.cycles_done += 1;
        Some(OutputIterItem {
            time: self.output.interval,
            amount: self.output.amount,
        })
    }
}
