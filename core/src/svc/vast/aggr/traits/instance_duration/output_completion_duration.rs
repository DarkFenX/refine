use super::instance_duration::InstanceDuration;
use crate::{
    num::{Count, PValue},
    svc::output::{Output, OutputComplex, OutputSimple},
};

impl<T> Output<T>
where
    T: Copy + InstanceDuration,
{
    pub(in crate::svc::vast) fn get_completion_duration(&self) -> PValue {
        match self {
            Output::Simple(inner) => inner.get_completion_duration(),
            Output::Complex(inner) => inner.get_completion_duration(),
        }
    }
}
impl<T> OutputSimple<T>
where
    T: Copy + InstanceDuration,
{
    pub(super) fn get_completion_duration(&self) -> PValue {
        self.delay + self.instance.get_duration()
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + InstanceDuration,
{
    pub(super) fn get_completion_duration(&self) -> PValue {
        if self.repeats < Count::ONE {
            return PValue::ZERO;
        };
        let interval_count = (self.repeats - Count::ONE).into_pvalue();
        let instance_duration = self.instance.get_duration();
        self.delay + self.interval.mul_add(interval_count, instance_duration)
    }
}
