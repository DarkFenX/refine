use super::proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_spool_cycle_output};
use crate::{
    num::{Count, PValue},
    svc::{
        cycle::{CycleIter, CycleSeq},
        output::Output,
        vast::aggr::traits::LimitInstance,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data entities - iter functions do not expose iterators right away to allow optimizations in
// aggregators of higher kinds
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) enum AggrIterData<T: Copy> {
    Regular(AggrIterDataRegular<T>),
    Spool(AggrIterDataSpool<T>),
}
impl<T: Copy> AggrIterData<T> {
    pub(in crate::svc::vast) fn iter(&self) -> AggrIter<T> {
        match self {
            Self::Regular(inner) => AggrIter::Regular(inner.iter()),
            Self::Spool(inner) => AggrIter::Spool(inner.iter()),
        }
    }
}

pub(in crate::svc::vast) struct AggrIterDataRegular<T: Copy> {
    pub(in crate::svc::vast) cseq: CycleSeq<AggrPartDataRegular<T>>,
}
impl<T: Copy> AggrIterDataRegular<T> {
    pub(super) fn new(cseq: CycleSeq<AggrPartDataRegular<T>>) -> Self {
        Self { cseq }
    }
    fn iter(&self) -> AggrIterRegular<T> {
        AggrIterRegular::new(self.cseq.iter_cycles())
    }
}

pub(in crate::svc::vast) struct AggrIterDataSpool<T: Copy> {
    pub(in crate::svc::vast) cseq: CycleSeq<AggrPartDataSpool<T>>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
}
impl<T: Copy> AggrIterDataSpool<T> {
    pub(super) fn new(
        cseq: CycleSeq<AggrPartDataSpool<T>>,
        inv_proj: AggrProjInvData<T>,
        inv_spool: AggrSpoolInvData,
    ) -> Self {
        Self {
            cseq,
            inv_proj,
            inv_spool,
        }
    }
    fn iter(&self) -> AggrIterSpool<T> {
        AggrIterSpool::new(self.cseq.iter_cycles(), self.inv_proj, self.inv_spool)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterItem<T: Copy> {
    pub(in crate::svc::vast) output: Output<T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}

pub(in crate::svc::vast) enum AggrIter<T: Copy> {
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
pub(in crate::svc::vast) struct AggrIterRegular<T: Copy> {
    cycle_iter: CycleIter<AggrPartDataRegular<T>>,
}
impl<T: Copy> AggrIterRegular<T> {
    fn new(cycle_iter: CycleIter<AggrPartDataRegular<T>>) -> Self {
        Self { cycle_iter }
    }
}
impl<T: Copy> Iterator for AggrIterRegular<T> {
    type Item = AggrIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle_iter.next().map(|v| AggrIterItem {
            output: v.output,
            cycle_duration: v.cycle_duration,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::vast) struct AggrPartDataRegular<T: Copy> {
    // Duration it takes per cycle in this part
    pub(in crate::svc::vast) cycle_duration: PValue,
    pub(in crate::svc::vast) output: Output<T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool variant of iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterSpool<T: Copy> {
    cycle_iter: CycleIter<AggrPartDataSpool<T>>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    uninterrupted_cycles: Count,
}
impl<T: Copy> AggrIterSpool<T> {
    fn new(
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
            output,
            cycle_duration: v.cycle_duration,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::vast) struct AggrPartDataSpool<T: Copy> {
    // Duration it takes per cycle in this part
    pub(in crate::svc::vast) cycle_duration: PValue,
    // Are there interrupts of any kind every cycle in this part
    pub(super) interrupt: bool,
    // Part-specific strength multiplier, which does not include spool factor
    pub(super) str_mult: PValue,
    pub(in crate::svc::vast) output_zero_spool: Output<T>,
    pub(super) output_max_spool: Output<T>,
}
