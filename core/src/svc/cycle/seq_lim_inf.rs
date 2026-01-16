use crate::{
    misc::InfCount,
    num::{Count, PValue},
    svc::cycle::{CSeqPart, CycleDataDur, CycleDataFull, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf},
    util::LibConvertExtend,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CSeqLimInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn convert<R>(self) -> CycleSeq<R>
    where
        R: From<T> + Eq,
    {
        let p1_data = R::from(self.p1_data);
        let p2_data = R::from(self.p2_data);
        match p1_data == p2_data {
            true => CycleSeq::Inf(CSeqInf { data: p1_data }),
            false => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
            }),
        }
    }
    pub(in crate::svc) fn convert_extend<X, R>(self, p1_xt: X, p2_xt: X) -> CycleSeq<R>
    where
        T: LibConvertExtend<X, R>,
        R: Eq,
    {
        let p1_data = self.p1_data.lib_convert_extend(p1_xt);
        let p2_data = self.p2_data.lib_convert_extend(p2_xt);
        match p1_data == p2_data {
            true => CycleSeq::Inf(CSeqInf { data: p1_data }),
            false => CycleSeq::LimInf(CSeqLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
            }),
        }
    }
}
impl<T> CSeqLimInf<T>
where
    T: Copy,
{
    pub(super) fn iter_cycles(&self) -> CSeqLimInfCycleIter<T> {
        CSeqLimInfCycleIter::new(*self)
    }
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqLimInfPartIter<'_, T> {
        CSeqLimInfPartIter::new(self)
    }
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(CSeqInf { data: self.p2_data }))
    }
}
impl CSeqLimInf {
    pub(super) fn get_average_duration(&self) -> PValue {
        self.p2_data.duration
    }
}
impl CSeqLimInf<CycleDataDur> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimInfCycleIter<T> {
    cseq: CSeqLimInf<T>,
    index: u8,
    p1_repeats_done: Count,
}
impl<T> CSeqLimInfCycleIter<T> {
    fn new(cseq: CSeqLimInf<T>) -> Self {
        Self {
            cseq,
            index: 0,
            p1_repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLimInfCycleIter<T>
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
            1 => Some(self.cseq.p2_data),
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence part iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimInfPartIter<'a, T> {
    cseq: &'a CSeqLimInf<T>,
    index: usize,
}
impl<'a, T> CSeqLimInfPartIter<'a, T> {
    fn new(cseq: &'a CSeqLimInf<T>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<T> Iterator for CSeqLimInfPartIter<'_, T>
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
                    repeat_count: InfCount::Infinite,
                })
            }
            2 => None,
            _ => unreachable!(),
        }
    }
}
