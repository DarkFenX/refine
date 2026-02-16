use super::shared::OutputInstanceIterItem;
use crate::num::{Count, PValue};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputSimple<T: Copy> {
    pub(crate) instance: T,
    pub(crate) delay: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Instance iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputSimple<T> {
    pub(super) fn into_instance_iter(self) -> OutputInstanceIterSimple<T> {
        OutputInstanceIterSimple::new(self)
    }
}

pub(in crate::svc) struct OutputInstanceIterSimple<T: Copy> {
    output: OutputSimple<T>,
    done: bool,
}
impl<T: Copy> OutputInstanceIterSimple<T> {
    fn new(output: OutputSimple<T>) -> Self {
        Self { output, done: false }
    }
}
impl<T: Copy> Iterator for OutputInstanceIterSimple<T> {
    type Item = OutputInstanceIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => None,
            false => {
                self.done = true;
                Some(OutputInstanceIterItem {
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
    pub(super) fn get_instance_count(&self) -> Count {
        Count::ONE
    }
    pub(super) fn get_immediate_instance(&self) -> Option<T> {
        match self.delay {
            PValue::ZERO => Some(self.instance),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
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
