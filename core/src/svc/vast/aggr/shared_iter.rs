use super::proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_spool_cycle_output};
use crate::{
    num::{Count, PValue},
    svc::{
        cycle::CycleIter,
        output::{Output, OutputInstanceIter},
        vast::aggr::traits::LimitInstance,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterItem<T>
where
    T: Copy,
{
    pub(in crate::svc::vast) instance_iter: OutputInstanceIter<T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}

pub(in crate::svc::vast) enum AggrIter<T>
where
    T: Copy,
{
    Regular(AggrIterRegular<T>),
    Spool(AggrIterSpool<T>),
}
impl<T> Iterator for AggrIter<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
{
    type Item = AggrIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Regular(inner) => inner.next(),
            Self::Spool(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-spool variant of iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterRegular<T>
where
    T: Copy,
{
    cycle_iter: CycleIter<AggrPartDataRegular<T>>,
}
impl<T> AggrIterRegular<T>
where
    T: Copy,
{
    pub(super) fn new(cycle_iter: CycleIter<AggrPartDataRegular<T>>) -> Self {
        Self { cycle_iter }
    }
}
impl<T> Iterator for AggrIterRegular<T>
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataRegular<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) cycle_duration: PValue,
    pub(super) output: Output<T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool variant of iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterSpool<T>
where
    T: Copy,
{
    cycle_iter: CycleIter<AggrPartDataSpool<T>>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    uninterrupted_cycles: Count,
}
impl<T> AggrIterSpool<T>
where
    T: Copy,
{
    pub(super) fn new(
        cycle_iter: CycleIter<AggrPartDataSpool<T>>,
        inv_proj: AggrProjInvData<T>,
        inv_spool: AggrSpoolInvData,
    ) -> Self {
        Self {
            cycle_iter,
            inv_proj,
            inv_spool,
            uninterrupted_cycles: Count::ZERO,
        }
    }
}
impl<T> Iterator for AggrIterSpool<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
{
    type Item = AggrIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let cycle_data = self.cycle_iter.next()?;
        let output = match self.uninterrupted_cycles {
            Count::ZERO => cycle_data.output_zero_spool,
            n if n >= self.inv_spool.cycles_to_max => cycle_data.output_max_spool,
            n => {
                let cycle_spool = self.inv_spool.calc_cycle_spool(n);
                get_proj_spool_cycle_output(&self.inv_proj, cycle_data.str_mult, cycle_spool)
            }
        };
        match cycle_data.interrupt {
            true => self.uninterrupted_cycles = Count::ZERO,
            false => self.uninterrupted_cycles += Count::ONE,
        }
        self.cycle_iter.next().map(|v| AggrIterItem {
            instance_iter: output.into_instance_iter(),
            cycle_duration: v.cycle_duration,
        })
    }
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
