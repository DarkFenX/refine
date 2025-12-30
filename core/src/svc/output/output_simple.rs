use ordered_float::Float;

use super::shared::OutputIterItem;
use crate::{def::AttrVal, util::FLOAT_TOLERANCE};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputSimple<T>
where
    T: Copy,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
}
impl<T> OutputSimple<T>
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
        self.delay
    }
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputSimpleAmountIter::new(self)
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + Default,
{
    pub(super) fn get_sum_by_time(&self, time: AttrVal) -> T {
        match self.delay <= time {
            true => self.amount,
            false => T::default(),
        }
    }
}
impl OutputSimple<AttrVal> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount.abs() > FLOAT_TOLERANCE
    }
    pub(super) fn absolute_impact(&self) -> AttrVal {
        self.amount.abs()
    }
    pub(super) fn add_amount(&mut self, amount: AttrVal) {
        self.amount += amount;
    }
}
impl<T> std::ops::Neg for OutputSimple<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.amount = -self.amount;
        self
    }
}
impl<T> std::ops::MulAssign<AttrVal> for OutputSimple<T>
where
    T: Copy + std::ops::MulAssign<AttrVal>,
{
    fn mul_assign(&mut self, rhs: AttrVal) {
        self.amount.mul_assign(rhs);
    }
}

struct OutputSimpleAmountIter<'a, T>
where
    T: Copy,
{
    output: &'a OutputSimple<T>,
    done: bool,
}
impl<'a, T> OutputSimpleAmountIter<'a, T>
where
    T: Copy,
{
    fn new(output: &'a OutputSimple<T>) -> Self {
        Self { output, done: false }
    }
}
impl<T> Iterator for OutputSimpleAmountIter<'_, T>
where
    T: Copy,
{
    type Item = OutputIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => None,
            false => {
                self.done = true;
                Some(OutputIterItem {
                    time: self.output.delay,
                    amount: self.output.amount,
                })
            }
        }
    }
}
