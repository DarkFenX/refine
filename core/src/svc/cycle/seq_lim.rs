use crate::{
    misc::{Count, InfCount, PValue},
    svc::cycle::{CSeqPart, CycleDataFull, CycleDataTime, CycleSeq, CycleSeqLooped},
    util::LibConvertExtend,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLim<T = CycleDataFull> {
    pub(in crate::svc) data: T,
    pub(in crate::svc) repeat_count: Count,
}
impl<T> CSeqLim<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.data
    }
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        None
    }
    pub(super) fn convert<R>(self) -> CycleSeq<R>
    where
        R: From<T>,
    {
        CycleSeq::Lim(CSeqLim {
            data: self.data.into(),
            repeat_count: self.repeat_count,
        })
    }
    pub(in crate::svc) fn convert_extend<X, R>(self, xt: X) -> CycleSeq<R>
    where
        T: LibConvertExtend<X, R>,
    {
        CycleSeq::Lim(CSeqLim {
            data: self.data.lib_convert_extend(xt),
            repeat_count: self.repeat_count,
        })
    }
}
impl<T> CSeqLim<T>
where
    T: Copy,
{
    pub(super) fn iter_cycles(&self) -> CSeqLimCycleIter<T> {
        CSeqLimCycleIter::new(*self)
    }
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqLimPartIter<'_, T> {
        CSeqLimPartIter::new(self)
    }
}
impl CSeqLim {
    pub(super) fn get_time(&self) -> PValue {
        self.data.time
    }
}
impl CSeqLim<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
            repeat_count: self.repeat_count,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimCycleIter<T> {
    cseq: CSeqLim<T>,
    repeats_done: Count,
}
impl<T> CSeqLimCycleIter<T> {
    fn new(cseq: CSeqLim<T>) -> Self {
        Self {
            cseq,
            repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLimCycleIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeats_done >= self.cseq.repeat_count {
            return None;
        }
        self.repeats_done += Count::ONE;
        Some(self.cseq.data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part data + iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimPartIter<'a, T> {
    cseq: &'a CSeqLim<T>,
    yielded: bool,
}
impl<'a, T> CSeqLimPartIter<'a, T> {
    fn new(cseq: &'a CSeqLim<T>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<T> Iterator for CSeqLimPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        Some(CSeqPart {
            data: self.cseq.data,
            repeat_count: InfCount::Count(self.cseq.repeat_count),
        })
    }
}
