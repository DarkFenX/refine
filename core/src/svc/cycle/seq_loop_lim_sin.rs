use crate::{
    misc::InfCount,
    num::Count,
    svc::cycle::{CSeqLoopedPart, CSeqPart, CycleDtHard, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLoopLimSin<T> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    // Optional hard downtime every loop
    pub(in crate::svc) dt_hard: Option<CycleDtHard>,
}
impl<T> CSeqLoopLimSin<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleDtHard> {
        self.dt_hard
    }
}
impl<T> CSeqLoopLimSin<T>
where
    T: Copy,
{
    pub(super) fn iter_cycles(&self) -> CSeqLoopLimSinCycleIter<T> {
        CSeqLoopLimSinCycleIter::new(*self)
    }
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqLoopLimSinPartIter<'_, T> {
        CSeqLoopLimSinPartIter::new(self)
    }
    pub(super) fn iter_cseq_parts_looped(&self) -> CSeqLoopedLoopLimSinPartIter<'_, T> {
        CSeqLoopedLoopLimSinPartIter::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLoopLimSin<T> {
    pub(super) fn convert<U>(self) -> CSeqLoopLimSin<U>
    where
        U: From<T>,
    {
        CSeqLoopLimSin {
            p1_data: U::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: U::from(self.p2_data),
            dt_hard: self.dt_hard,
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqLoopLimSin<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqLoopLimSin {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
            dt_hard: self.dt_hard,
        }
    }
    pub(super) fn optimize(self) -> CycleSeq<T>
    where
        T: Eq,
    {
        match self.p1_data == self.p2_data && self.dt_hard.is_none() {
            true => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                dt_hard: None,
            }),
            false => CycleSeq::LoopLimSin(self),
        }
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<T>
    where
        T: Eq,
    {
        match self.p1_data == self.p2_data && self.dt_hard.is_none() {
            true => CycleSeqLooped::Inf(CSeqInf {
                data: self.p1_data,
                dt_hard: None,
            }),
            false => CycleSeqLooped::LoopLimSin(self),
        }
    }
}
impl<T> CSeqLoopLimSin<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::LoopLimSin(*self))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLoopLimSinCycleIter<T> {
    cseq: CSeqLoopLimSin<T>,
    p1_repeats_done: Count,
}
impl<T> CSeqLoopLimSinCycleIter<T> {
    fn new(cseq: CSeqLoopLimSin<T>) -> Self {
        Self {
            cseq,
            p1_repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLoopLimSinCycleIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p1_repeats_done >= self.cseq.p1_repeat_count {
            self.p1_repeats_done = Count::ZERO;
            return Some(self.cseq.p2_data);
        }
        self.p1_repeats_done += Count::ONE;
        Some(self.cseq.p1_data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence part iterators
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLoopLimSinPartIter<'a, T> {
    cseq: &'a CSeqLoopLimSin<T>,
    index: usize,
}
impl<'a, T> CSeqLoopLimSinPartIter<'a, T> {
    fn new(cseq: &'a CSeqLoopLimSin<T>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<T> Iterator for CSeqLoopLimSinPartIter<'_, T>
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
            2 => None,
            _ => unreachable!(),
        }
    }
}

pub(in crate::svc) struct CSeqLoopedLoopLimSinPartIter<'a, T> {
    cseq: &'a CSeqLoopLimSin<T>,
    index: usize,
}
impl<'a, T> CSeqLoopedLoopLimSinPartIter<'a, T> {
    fn new(cseq: &'a CSeqLoopLimSin<T>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<T> Iterator for CSeqLoopedLoopLimSinPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqLoopedPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index = 1;
                Some(CSeqLoopedPart {
                    data: self.cseq.p1_data,
                    repeat_count: self.cseq.p1_repeat_count,
                })
            }
            1 => {
                self.index = 2;
                Some(CSeqLoopedPart {
                    data: self.cseq.p2_data,
                    repeat_count: Count::ONE,
                })
            }
            2 => None,
            _ => unreachable!(),
        }
    }
}
