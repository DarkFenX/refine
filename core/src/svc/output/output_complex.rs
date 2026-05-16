use super::shared::OutputInstanceIterItem;
use crate::num::{Count, PValue};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<T>
where
    T: Copy,
{
    pub(crate) instance: T,
    pub(crate) delay: PValue,
    // Total count of instances
    pub(crate) repeats: Count,
    pub(crate) interval: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> OutputComplex<T>
where
    T: Copy,
{
    pub(super) fn get_instance(&self) -> T {
        self.instance
    }
    pub(super) fn get_instance_count(&self) -> Count {
        self.repeats
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
impl<T> OutputComplex<T>
where
    T: Copy + std::ops::MulAssign<PValue>,
{
    pub(super) fn instance_mul_assign(&mut self, rhs: PValue) {
        self.instance.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Instance iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> OutputComplex<T>
where
    T: Copy,
{
    pub(super) fn into_instance_iter(self) -> OutputInstanceIterComplex<T> {
        OutputInstanceIterComplex::new(self)
    }
}

pub(in crate::svc) struct OutputInstanceIterComplex<T>
where
    T: Copy,
{
    output: OutputComplex<T>,
    cycles_done: Count,
}
impl<T> OutputInstanceIterComplex<T>
where
    T: Copy,
{
    fn new(output: OutputComplex<T>) -> Self {
        Self {
            output,
            cycles_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for OutputInstanceIterComplex<T>
where
    T: Copy,
{
    type Item = OutputInstanceIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.output.repeats {
            return None;
        }
        if self.cycles_done == Count::ZERO {
            self.cycles_done += Count::ONE;
            return Some(OutputInstanceIterItem {
                time_passed: self.output.delay,
                instance: self.output.instance,
            });
        }
        self.cycles_done += Count::ONE;
        Some(OutputInstanceIterItem {
            time_passed: self.output.interval,
            instance: self.output.instance,
        })
    }
}
