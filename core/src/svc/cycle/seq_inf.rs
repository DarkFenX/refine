use crate::{
    misc::InfCount,
    num::Count,
    svc::cycle::{CSeqLoopedPart, CSeqPart, CycleHardDt, CycleSeq, CycleSeqLooped},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqInf<T> {
    pub(in crate::svc) data: T,
    // Optional hard downtime every cycle
    pub(in crate::svc) hard_dt: Option<CycleHardDt>,
}
impl<T> CSeqInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleHardDt> {
        self.hard_dt
    }
}
impl<T> CSeqInf<T>
where
    T: Copy,
{
    pub(super) fn iter_cycles(&self) -> CSeqInfCycleIter<T> {
        CSeqInfCycleIter::new(*self)
    }
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqInfPartIter<'_, T> {
        CSeqInfPartIter::new(self)
    }
    pub(super) fn iter_cseq_parts_looped(&self) -> CSeqLoopedInfPartIter<'_, T> {
        CSeqLoopedInfPartIter::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqInf<T> {
    pub(super) fn convert<U>(self) -> CSeqInf<U>
    where
        U: From<T>,
    {
        CSeqInf {
            data: self.data.into(),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqInf<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqInf {
            data: converter.lib_convert(self.data),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T> {
        CycleSeq::Inf(self)
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<T> {
        CycleSeqLooped::Inf(self)
    }
}
impl<T> CSeqInf<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(*self))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqInfCycleIter<T> {
    cseq: CSeqInf<T>,
}
impl<T> CSeqInfCycleIter<T> {
    fn new(cseq: CSeqInf<T>) -> Self {
        Self { cseq }
    }
}
impl<T> Iterator for CSeqInfCycleIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.cseq.data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence part iterators
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqInfPartIter<'a, T> {
    cseq: &'a CSeqInf<T>,
    yielded: bool,
}
impl<'a, T> CSeqInfPartIter<'a, T> {
    fn new(cseq: &'a CSeqInf<T>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<T> Iterator for CSeqInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CSeqPart {
            data: self.cseq.data,
            repeat_count: InfCount::Infinite,
        })
    }
}

pub(in crate::svc) struct CSeqLoopedInfPartIter<'a, T> {
    cseq: &'a CSeqInf<T>,
    yielded: bool,
}
impl<'a, T> CSeqLoopedInfPartIter<'a, T> {
    fn new(cseq: &'a CSeqInf<T>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<T> Iterator for CSeqLoopedInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqLoopedPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CSeqLoopedPart {
            data: self.cseq.data,
            repeat_count: Count::ONE,
        })
    }
}
