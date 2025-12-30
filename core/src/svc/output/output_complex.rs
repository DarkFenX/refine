use ordered_float::Float;

use super::shared::OutputIterItem;
use crate::{
    def::{AttrVal, Count, OF},
    util::FLOAT_TOLERANCE,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<T>
where
    T: Copy,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
    pub(crate) repeats: Count,
    pub(crate) interval: AttrVal,
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
    pub(super) fn get_completion_time(&self) -> AttrVal {
        if self.repeats < 1 {
            return OF(0.0);
        };
        self.delay + self.interval * (self.repeats - 1) as f64
    }
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputComplexAmountIter::new(self)
    }
}
impl OutputComplex<AttrVal> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount.abs() > FLOAT_TOLERANCE
    }
    pub(super) fn absolute_impact(&self) -> AttrVal {
        self.amount.abs() * self.repeats as f64
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
    cycles_done: Count,
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
