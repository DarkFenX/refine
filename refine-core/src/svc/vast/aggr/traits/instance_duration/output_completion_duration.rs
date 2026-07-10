use super::instance_duration::InstanceDuration;
use crate::{
    num::{Count, PValue},
    svc::output::{Output, OutputComplex, OutputSimple},
};

impl<I> Output<I> {
    pub(in crate::svc::vast) fn get_completion_duration(&self) -> PValue
    where
        I: InstanceDuration,
    {
        match self {
            Output::Simple(inner) => inner.get_completion_duration(),
            Output::Complex(inner) => inner.get_completion_duration(),
        }
    }
}
impl<I> OutputSimple<I> {
    pub(super) fn get_completion_duration(&self) -> PValue
    where
        I: InstanceDuration,
    {
        self.delay + self.instance.get_duration()
    }
}
impl<I> OutputComplex<I> {
    pub(super) fn get_completion_duration(&self) -> PValue
    where
        I: InstanceDuration,
    {
        if self.repeats < Count::ONE {
            return PValue::ZERO;
        };
        let interval_count = (self.repeats - Count::ONE).into_pvalue();
        let instance_duration = self.instance.get_duration();
        self.delay + self.interval.mul_add(interval_count, instance_duration)
    }
}
