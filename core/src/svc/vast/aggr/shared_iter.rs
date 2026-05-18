use super::{
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_spool_cycle_output},
    shared::{AggrHardDtSimple, AggrPartData},
    traits::{InstanceDuration, InstanceLimit},
};
use crate::{
    num::{Count, PValue},
    svc::{
        cycle::{CycleIter, CycleSeq},
        output::Output,
        traits::GetDuration,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data entities - iter functions do not expose iterators right away to allow optimizations in
// aggregators of higher kinds
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) enum AggrIterData<I>
where
    I: Copy,
{
    Regular(AggrIterDataRegular<I>),
    Spool(AggrIterDataSpool<I>),
}
impl<I> AggrIterData<I>
where
    I: Copy,
{
    pub(in crate::svc::vast) fn iter(&self) -> AggrIter<I> {
        match self {
            Self::Regular(inner) => AggrIter::Regular(inner.iter()),
            Self::Spool(inner) => AggrIter::Spool(inner.iter()),
        }
    }
}

pub(in crate::svc::vast) struct AggrIterDataRegular<I>
where
    I: Copy,
{
    pub(in crate::svc::vast) cseq: CycleSeq<AggrPartData<I>, AggrHardDtSimple>,
}
impl<I> AggrIterDataRegular<I>
where
    I: Copy,
{
    pub(super) fn new(cseq: CycleSeq<AggrPartData<I>, AggrHardDtSimple>) -> Self {
        Self { cseq }
    }
    fn iter(&self) -> AggrIterRegular<I> {
        AggrIterRegular::new(self.cseq.iter_cycles())
    }
}

pub(in crate::svc::vast) struct AggrIterDataSpool<I>
where
    I: Copy,
{
    pub(in crate::svc::vast) cseq: CycleSeq<AggrPartDataSpool<I>, AggrHardDtSimple>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
}
impl<I> AggrIterDataSpool<I>
where
    I: Copy,
{
    pub(super) fn new(
        cseq: CycleSeq<AggrPartDataSpool<I>, AggrHardDtSimple>,
        inv_proj: AggrProjInvData<I>,
        inv_spool: AggrSpoolInvData,
    ) -> Self {
        Self {
            cseq,
            inv_proj,
            inv_spool,
        }
    }
    fn iter(&self) -> AggrIterSpool<I> {
        AggrIterSpool::new(self.cseq.iter_cycles(), self.inv_proj, self.inv_spool)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterItem<I>
where
    I: Copy,
{
    pub(in crate::svc::vast) output: Output<I>,
    // Is set only if output completion duration is shorter than time left until hard downtime
    pub(in crate::svc::vast) output_duration_limit: Option<PValue>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}

pub(in crate::svc::vast) enum AggrIter<I>
where
    I: Copy,
{
    Regular(AggrIterRegular<I>),
    Spool(AggrIterSpool<I>),
}
impl<I> Iterator for AggrIter<I>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    type Item = AggrIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Regular(inner) => inner.next(),
            Self::Spool(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator for simple cases
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterRegular<I>
where
    I: Copy,
{
    cycle_iter: CycleIter<AggrPartData<I>>,
}
impl<I> AggrIterRegular<I>
where
    I: Copy,
{
    fn new(cycle_iter: CycleIter<AggrPartData<I>>) -> Self {
        Self { cycle_iter }
    }
}
impl<I> Iterator for AggrIterRegular<I>
where
    I: Copy + InstanceDuration,
{
    type Item = AggrIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle_iter.next().map(|v| {
            let output_duration_limit = get_output_duration_limit(v.time_until_hard_dt, &v.data.output);
            let cycle_duration = get_full_cycle_duration(v.data.cycle_main_duration, v.hard_dt_duration);
            AggrIterItem {
                output: v.data.output,
                output_duration_limit,
                cycle_duration,
            }
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool variant of iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct AggrIterSpool<I>
where
    I: Copy,
{
    cycle_iter: CycleIter<AggrPartDataSpool<I>>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    uninterrupted_cycles: Count,
}
impl<I> AggrIterSpool<I>
where
    I: Copy,
{
    fn new(
        cycle_iter: CycleIter<AggrPartDataSpool<I>>,
        inv_proj: AggrProjInvData<I>,
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
impl<I> Iterator for AggrIterSpool<I>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    type Item = AggrIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let cycle_data = self.cycle_iter.next()?;
        let output = match self.uninterrupted_cycles {
            Count::ZERO => cycle_data.data.output_zero_spool,
            n if n >= self.inv_spool.cycles_to_max => cycle_data.data.output_max_spool,
            n => {
                let cycle_spool = self.inv_spool.calc_cycle_spool(n);
                get_proj_spool_cycle_output(&self.inv_proj, cycle_data.data.str_mult, cycle_spool)
            }
        };
        match cycle_data.data.interrupt || cycle_data.hard_dt_duration.is_some() {
            true => self.uninterrupted_cycles = Count::ZERO,
            false => self.uninterrupted_cycles += Count::ONE,
        }
        let output_duration_limit = get_output_duration_limit(cycle_data.time_until_hard_dt, &output);
        let cycle_duration = get_full_cycle_duration(cycle_data.data.cycle_main_duration, cycle_data.hard_dt_duration);
        Some(AggrIterItem {
            output,
            output_duration_limit,
            cycle_duration,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::vast) struct AggrPartDataSpool<I>
where
    I: Copy,
{
    // Active + soft downtime duration combined
    pub(in crate::svc::vast) cycle_main_duration: PValue,
    // Are there interrupts of any kind every cycle in this part
    pub(super) interrupt: bool,
    // Part-specific strength multiplier, which does not include spool factor
    pub(super) str_mult: PValue,
    pub(in crate::svc::vast) output_zero_spool: Output<I>,
    pub(super) output_max_spool: Output<I>,
}
impl<I> GetDuration for AggrPartDataSpool<I>
where
    I: Copy,
{
    fn get_duration(&self) -> PValue {
        self.cycle_main_duration
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_output_duration_limit<I>(time_until_hard_dt: Option<PValue>, output: &Output<I>) -> Option<PValue>
where
    I: Copy + InstanceDuration,
{
    match time_until_hard_dt {
        Some(time_until_hard_dt) if output.get_completion_duration() > time_until_hard_dt => Some(time_until_hard_dt),
        _ => None,
    }
}

fn get_full_cycle_duration(mut main_duration: PValue, hard_dt_duration: Option<PValue>) -> PValue {
    if let Some(hard_dt_duration) = hard_dt_duration {
        main_duration += hard_dt_duration;
    }
    main_duration
}
