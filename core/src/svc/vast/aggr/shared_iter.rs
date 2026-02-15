use crate::{
    num::PValue,
    svc::{
        cycle::CycleIter,
        output::{Output, OutputInstanceIter},
    },
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataRegular<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) cycle_duration: PValue,
    pub(super) output: Output<T>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataSpool<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) cycle_duration: PValue,
    // Are there interrupts of any kind every cycle in this part
    pub(super) interrupt: bool,
    // Part-specific strength multiplier, which does not include spool factor
    pub(super) str_mult: PValue,
    pub(super) output_zero_spool: Output<T>,
    pub(super) output_max_spool: Output<T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregated iterator and its yielded item
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIter<T>
where
    T: Copy,
{
    cycle_iter: CycleIter<AggrPartDataRegular<T>>,
}
impl<T> AggrIter<T>
where
    T: Copy,
{
    pub(super) fn new(cycle_iter: CycleIter<AggrPartDataRegular<T>>) -> Self {
        Self { cycle_iter }
    }
}
impl<T> Iterator for AggrIter<T>
where
    T: Copy,
{
    type Item = AggrIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle_iter.next().map(|v| AggrIterItem {
            instance_iter: v.output.into_instance_iter(),
            cycle_duration: v.cycle_duration,
        })
    }
}

pub(in crate::svc::vast) struct AggrIterItem<T>
where
    T: Copy,
{
    pub(in crate::svc::vast) instance_iter: OutputInstanceIter<T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}
