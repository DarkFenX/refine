use super::{output::Output, shared::OutputInstanceIterItem};
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
    pub(super) fn limit_duration(&self, duration: PValue) -> Option<Output<T>> {
        match duration >= self.delay {
            true => Some(Output::Simple(*self)),
            false => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> OutputSimple<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    pub(super) fn instance_mul_assign(&mut self, rhs: PValue) {
        self.instance.mul_assign(rhs);
    }
}
