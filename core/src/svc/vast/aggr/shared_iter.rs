use crate::{num::PValue, svc::output::OutputInstanceIter};

pub(in crate::svc::vast) struct AggrIterItem<'a, T>
where
    T: Copy,
{
    pub(in crate::svc::vast) instance_iter: OutputInstanceIter<'a, T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}
