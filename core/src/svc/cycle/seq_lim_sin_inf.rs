use crate::{
    misc::InfCount,
    num::{Count, PValue},
    svc::cycle::{
        CSeqPart, CycleDataFull, CycleDataTime, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf, seq_lim_inf::CSeqLimInf,
    },
    util::LibConvertExtend,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimSinInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    pub(in crate::svc) p3_data: T,
}
impl<T> CSeqLimSinInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn convert<R>(self) -> CycleSeq<R>
    where
        R: From<T> + Eq,
    {
        let p1_data = R::from(self.p1_data);
        let p2_data = R::from(self.p2_data);
        let p3_data = R::from(self.p3_data);
        match (p1_data == p2_data, p2_data == p3_data) {
            // Nothing to merge
            (false, false) => CycleSeq::LimSinInf(CSeqLimSinInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
                p3_data,
            }),
            // Merge part 2 into tail
            (false, true) => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: p3_data,
            }),
            // Merge part 2 into head
            (true, false) => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count + Count::ONE,
                p2_data: p3_data,
            }),
            // Whole sequence becomes a simple infinity
            (true, true) => CycleSeq::Inf(CSeqInf { data: p1_data }),
        }
    }
    pub(in crate::svc) fn convert_extend<X, R>(self, p1_xt: X, p2_xt: X, p3_xt: X) -> CycleSeq<R>
    where
        T: LibConvertExtend<X, R>,
        R: Eq,
    {
        let p1_data = self.p1_data.lib_convert_extend(p1_xt);
        let p2_data = self.p2_data.lib_convert_extend(p2_xt);
        let p3_data = self.p3_data.lib_convert_extend(p3_xt);
        match (p1_data == p2_data, p2_data == p3_data) {
            // Nothing to merge
            (false, false) => CycleSeq::LimSinInf(CSeqLimSinInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
                p3_data,
            }),
            // Merge part 2 into tail
            (false, true) => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: p3_data,
            }),
            // Merge part 2 into head
            (true, false) => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count + Count::ONE,
                p2_data: p3_data,
            }),
            // Whole sequence becomes a simple infinity
            (true, true) => CycleSeq::Inf(CSeqInf { data: p1_data }),
        }
    }
}
impl<T> CSeqLimSinInf<T>
where
    T: Copy,
{
    pub(super) fn iter_cycles(&self) -> CSeqLimSinInfCycleIter<T> {
        CSeqLimSinInfCycleIter::new(*self)
    }
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqLimSinInfPartIter<'_, T> {
        CSeqLimSinInfPartIter::new(self)
    }
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(CSeqInf { data: self.p3_data }))
    }
}
impl CSeqLimSinInf {
    pub(super) fn get_average_time(&self) -> PValue {
        self.p3_data.time
    }
}
impl CSeqLimSinInf<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
            p3_data: self.p3_data.copy_rounded(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimSinInfCycleIter<T> {
    cseq: CSeqLimSinInf<T>,
    index: u8,
    p1_repeats_done: Count,
}
impl<T> CSeqLimSinInfCycleIter<T> {
    fn new(cseq: CSeqLimSinInf<T>) -> Self {
        Self {
            cseq,
            index: 0,
            p1_repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLimSinInfCycleIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                if self.p1_repeats_done >= self.cseq.p1_repeat_count {
                    self.index = 1;
                    return Some(self.cseq.p2_data);
                }
                self.p1_repeats_done += Count::ONE;
                Some(self.cseq.p1_data)
            }
            1 => Some(self.cseq.p3_data),
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence part iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimSinInfPartIter<'a, T> {
    cseq: &'a CSeqLimSinInf<T>,
    index: usize,
}
impl<'a, T> CSeqLimSinInfPartIter<'a, T> {
    fn new(cseq: &'a CSeqLimSinInf<T>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<T> Iterator for CSeqLimSinInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index = 1;
                Some(CSeqPart {
                    data: self.cseq.p1_data,
                    repeat_count: InfCount::Count(self.cseq.p1_repeat_count),
                })
            }
            1 => {
                self.index = 2;
                Some(CSeqPart {
                    data: self.cseq.p2_data,
                    repeat_count: InfCount::Count(Count::ONE),
                })
            }
            2 => {
                self.index = 3;
                Some(CSeqPart {
                    data: self.cseq.p2_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            3 => None,
            _ => unreachable!(),
        }
    }
}
