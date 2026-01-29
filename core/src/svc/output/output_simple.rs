use super::shared::OutputIterItem;
use crate::num::{PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputSimple<T: Copy> {
    pub(crate) instance: T,
    pub(crate) delay: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSimple<T> {
    pub(super) fn iter_instances(&self) -> impl Iterator<Item = OutputIterItem<T>> {
        OutputSimpleInstanceIter::new(self)
    }
}

struct OutputSimpleInstanceIter<'a, T: Copy> {
    output: &'a OutputSimple<T>,
    done: bool,
}
impl<'a, T: Copy> OutputSimpleInstanceIter<'a, T> {
    fn new(output: &'a OutputSimple<T>) -> Self {
        Self { output, done: false }
    }
}
impl<T: Copy> Iterator for OutputSimpleInstanceIter<'_, T> {
    type Item = OutputIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => None,
            false => {
                self.done = true;
                Some(OutputIterItem {
                    time_passed: self.output.delay,
                    instance: self.output.instance,
                })
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSimple<T> {
    pub(super) fn get_instance(&self) -> T {
        self.instance
    }
    pub(super) fn get_max_instance(&self) -> T {
        self.instance
    }
}
impl OutputSimple<Value> {
    pub(super) fn get_absolute_impact(&self) -> PValue {
        self.instance.abs()
    }
    pub(super) fn add_instance(&mut self, instance: Value) {
        self.instance += instance;
    }
}
impl OutputSimple<PValue> {
    pub(super) fn has_impact(&self) -> bool {
        self.instance > PValue::FLOAT_TOLERANCE
    }
}
impl<T> std::ops::Mul<PValue> for OutputSimple<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            instance: self.instance * rhs,
            delay: self.delay,
        }
    }
}
impl<T> std::ops::MulAssign<PValue> for OutputSimple<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    fn mul_assign(&mut self, rhs: PValue) {
        self.instance.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions of inner type
////////////////////////////////////////////////////////////////////////////////////////////////////
impl OutputSimple<PValue> {
    pub(super) fn into_value(self) -> OutputSimple<Value> {
        OutputSimple {
            instance: self.instance.into_value(),
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
            instance: -self.instance,
            delay: self.delay,
        }
    }
}
