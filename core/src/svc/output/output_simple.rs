use ordered_float::Float;

use super::shared::OutputIterItem;
use crate::misc::{PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputSimple<T>
where
    T: Copy,
{
    pub(crate) amount: T,
    pub(crate) delay: PValue,
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
    pub(super) fn get_completion_time(&self) -> PValue {
        self.delay
    }
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputSimpleAmountIter::new(self)
    }
}
impl OutputSimple<Value> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount.abs() > PValue::FLOAT_TOLERANCE
    }
    pub(super) fn absolute_impact(&self) -> PValue {
        self.amount.abs()
    }
    pub(super) fn add_amount(&mut self, amount: Value) {
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
impl<T> std::ops::Mul<Value> for OutputSimple<T>
where
    T: Copy + std::ops::Mul<Value, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            delay: self.delay,
        }
    }
}
impl<T> std::ops::MulAssign<Value> for OutputSimple<T>
where
    T: Copy + std::ops::MulAssign<Value>,
{
    fn mul_assign(&mut self, rhs: Value) {
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
