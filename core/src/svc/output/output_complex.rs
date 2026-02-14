use super::shared::OutputInstanceIterItem;
use crate::num::{Count, PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<T: Copy> {
    pub(crate) instance: T,
    pub(crate) delay: PValue,
    // Total count of instances
    pub(crate) repeats: Count,
    pub(crate) interval: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Instance iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputComplex<T> {
    pub(super) fn into_instance_iter(self) -> OutputInstanceIterComplex<T> {
        OutputInstanceIterComplex::new(self)
    }
}

pub(in crate::svc) struct OutputInstanceIterComplex<T: Copy> {
    output: OutputComplex<T>,
    cycles_done: Count,
}
impl<T: Copy> OutputInstanceIterComplex<T> {
    fn new(output: OutputComplex<T>) -> Self {
        Self {
            output,
            cycles_done: Count::ZERO,
        }
    }
}
impl<T: Copy> Iterator for OutputInstanceIterComplex<T> {
    type Item = OutputInstanceIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.output.repeats {
            return None;
        }
        self.cycles_done += Count::ONE;
        Some(OutputInstanceIterItem {
            time_passed: self.output.interval,
            instance: self.output.instance,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: Copy> OutputComplex<T> {
    pub(super) fn get_instance(&self) -> T {
        self.instance
    }
    pub(super) fn get_instance_count(&self) -> Count {
        self.repeats
    }
}
impl OutputComplex<Value> {
    pub(super) fn get_absolute_impact(&self) -> PValue {
        self.instance.abs() * PValue::from_f64_unchecked(self.repeats.into_u32() as f64)
    }
    pub(super) fn add_instance(&mut self, instance: Value) {
        self.instance += instance;
    }
}
impl OutputComplex<PValue> {
    pub(super) fn has_impact(&self) -> bool {
        self.instance > PValue::FLOAT_TOLERANCE
    }
}
impl<T> std::ops::Mul<PValue> for OutputComplex<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            instance: self.instance * rhs,
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
        self.instance.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions of inner type
////////////////////////////////////////////////////////////////////////////////////////////////////
impl OutputComplex<PValue> {
    pub(super) fn into_value(self) -> OutputComplex<Value> {
        OutputComplex {
            instance: self.instance.into_value(),
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
            instance: -self.instance,
            delay: self.delay,
            repeats: self.repeats,
            interval: self.interval,
        }
    }
}
