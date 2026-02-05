use crate::num::{Count, PValue};

pub(in crate::svc::vast) struct SeqAccum<A> {
    pub(in crate::svc::vast) instances: A,
    pub(in crate::svc::vast) time: PValue,
}
impl<A> SeqAccum<A> {
    pub(in crate::svc::vast) fn add_instance<T>(&mut self, instance: T, chance_mult: Option<PValue>, count: Count)
    where
        A: SeqInstanceAccum<T>,
    {
        self.instances.add_instance(instance, chance_mult, count);
    }
    pub(in crate::svc::vast::aggr) fn merge_instance_accum<T>(&mut self, other: &A, count: Count)
    where
        A: SeqInstanceAccum<T>,
    {
        self.instances.merge(other, count);
    }
}

pub(in crate::svc::vast) trait SeqInstanceAccum<T> {
    fn add_instance(&mut self, instance: T, chance_mult: Option<PValue>, count: Count);
    fn merge(&mut self, other: &Self, count: Count);
}
