use super::{output::Output, output_simple::OutputSimple, shared::OutputInstanceIterItem};
use crate::num::{Count, PValue};

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
    pub(super) fn get_immediate_instance(&self) -> Option<T> {
        match self.delay {
            PValue::ZERO => Some(self.instance),
            _ => None,
        }
    }
    pub(super) fn limit_duration(&self, duration: PValue) -> Option<Output<T>> {
        match duration >= self.delay {
            true => {
                let post_delay = PValue::from_value_unchecked(duration - self.delay);
                let repeats = self
                    .repeats
                    .min(Count::ONE + Count::from_pvalue_trunced(post_delay / self.interval));
                match repeats {
                    Count::ONE => Some(Output::Simple(OutputSimple {
                        instance: self.instance,
                        delay: self.delay,
                    })),
                    repeats => Some(Output::Complex(OutputComplex {
                        instance: self.instance,
                        delay: self.delay,
                        repeats,
                        interval: self.interval,
                    })),
                }
            }
            false => None,
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
