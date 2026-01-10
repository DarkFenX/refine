use super::shared::OutputIterItem;
use crate::num::{PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputSimple<T: Copy> {
    pub(crate) amount: T,
    pub(crate) delay: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSimple<T> {
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputSimpleAmountIter::new(self)
    }
}

struct OutputSimpleAmountIter<'a, T: Copy> {
    output: &'a OutputSimple<T>,
    done: bool,
}
impl<'a, T: Copy> OutputSimpleAmountIter<'a, T> {
    fn new(output: &'a OutputSimple<T>) -> Self {
        Self { output, done: false }
    }
}
impl<T: Copy> Iterator for OutputSimpleAmountIter<'_, T> {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSimple<T> {
    pub(super) fn get_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_max_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_completion_time(&self) -> PValue {
        self.delay
    }
}
impl OutputSimple<Value> {
    pub(super) fn get_absolute_impact(&self) -> PValue {
        self.amount.abs()
    }
    pub(super) fn add_amount(&mut self, amount: Value) {
        self.amount += amount;
    }
}
impl OutputSimple<PValue> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount > PValue::FLOAT_TOLERANCE
    }
}
impl<T> std::ops::Mul<PValue> for OutputSimple<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            delay: self.delay,
        }
    }
}
impl<T> std::ops::MulAssign<PValue> for OutputSimple<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    fn mul_assign(&mut self, rhs: PValue) {
        self.amount.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions of inner type
////////////////////////////////////////////////////////////////////////////////////////////////////
impl OutputSimple<PValue> {
    pub(super) fn into_value(self) -> OutputSimple<Value> {
        OutputSimple {
            amount: self.amount.into_value(),
            delay: self.delay,
        }
    }
}
impl<T, U> std::ops::Neg for OutputSimple<T>
where
    T: Copy + std::ops::Neg<Output = U>,
    U: Copy,
{
    type Output = OutputSimple<U>;

    fn neg(self) -> Self::Output {
        OutputSimple {
            amount: -self.amount,
            delay: self.delay,
        }
    }
}
