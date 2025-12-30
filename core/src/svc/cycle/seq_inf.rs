use crate::{
    def::AttrVal,
    svc::cycle::{CSeqLoopedPart, CSeqPart, CycleDataFull, CycleDataTime, CycleSeq, CycleSeqLooped},
    util::{ConvertExtend, InfCount},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqInf<T = CycleDataFull> {
    pub(in crate::svc) data: T,
}
impl<T> CSeqInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.data
    }
    pub(super) fn convert<R>(self) -> CycleSeq<R>
    where
        R: From<T>,
    {
        CycleSeq::Inf(CSeqInf { data: self.data.into() })
    }
    pub(in crate::svc) fn convert_extend<X, R>(self, xt: X) -> CycleSeq<R>
    where
        T: ConvertExtend<X, R>,
    {
        CycleSeq::Inf(CSeqInf {
            data: self.data.convert_extend(xt),
        })
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
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(*self))
    }
}
impl CSeqInf {
    pub(super) fn get_time(&self) -> AttrVal {
        self.data.time
    }
}
impl CSeqInf<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
        }
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
            repeat_count: 1,
        })
    }
}
