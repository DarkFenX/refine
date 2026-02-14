use crate::{
    num::PValue,
    svc::output::{Output, OutputInstanceIter},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartData<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) cycle_duration: PValue,
    pub(super) output: Output<T>,
}

pub(in crate::svc::vast) struct AggrIterItem<T>
where
    T: Copy,
{
    pub(in crate::svc::vast) instance_iter: OutputInstanceIter<T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}
