use super::shared::OutputIterItem;
use crate::num::{Count, PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<T: Copy> {
    pub(crate) amount: T,
    pub(crate) delay: PValue,
    // Total count of times amount is output
    pub(crate) repeats: Count,
    pub(crate) interval: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputComplex<T> {
    pub(super) fn iter_amounts(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputComplexAmountIter::new(self)
    }
}

struct OutputComplexAmountIter<'a, T: Copy> {
    output: &'a OutputComplex<T>,
    cycles_done: Count,
}
impl<'a, T: Copy> OutputComplexAmountIter<'a, T> {
    fn new(output: &'a OutputComplex<T>) -> Self {
        Self {
            output,
            cycles_done: Count::ZERO,
        }
    }
}
impl<T: Copy> Iterator for OutputComplexAmountIter<'_, T> {
    type Item = OutputIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.output.repeats {
            return None;
        }
        self.cycles_done += Count::ONE;
        Some(OutputIterItem {
            time_passed: self.output.interval,
            amount: self.output.amount,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputComplex<T> {
    pub(super) fn get_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_max_amount(&self) -> T {
        self.amount
    }
}
impl OutputComplex<Value> {
    pub(super) fn get_absolute_impact(&self) -> PValue {
        self.amount.abs() * PValue::from_f64_unchecked(self.repeats.into_u32() as f64)
    }
    pub(super) fn add_amount(&mut self, amount: Value) {
        self.amount += amount;
    }
}
impl OutputComplex<PValue> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount > PValue::FLOAT_TOLERANCE
    }
}
impl<T> std::ops::Mul<PValue> for OutputComplex<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            delay: self.delay,
            repeats: self.repeats,
            interval: self.interval,
        }
    }
}
impl<T> std::ops::MulAssign<PValue> for OutputComplex<T>
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
impl OutputComplex<PValue> {
    pub(super) fn into_value(self) -> OutputComplex<Value> {
        OutputComplex {
            amount: self.amount.into_value(),
            delay: self.delay,
            repeats: self.repeats,
            interval: self.interval,
        }
    }
}
impl<T, U> std::ops::Neg for OutputComplex<T>
where
    T: Copy + std::ops::Neg<Output = U>,
    U: Copy,
{
    type Output = OutputComplex<U>;

    fn neg(self) -> Self::Output {
        OutputComplex {
            amount: -self.amount,
            delay: self.delay,
            repeats: self.repeats,
            interval: self.interval,
        }
    }
}
