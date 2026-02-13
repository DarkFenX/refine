use crate::{
    misc::InfCount,
    num::Count,
    svc::cycle::{CSeqLoopedPart, CSeqPart, CycleDataDur, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleSeqLoopLimSin<T> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CycleSeqLoopLimSin<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn convert_and_optimize<U>(self) -> CycleSeq<U>
    where
        U: From<T> + Eq,
    {
        let p1_data_conv = U::from(self.p1_data);
        let p2_data_conv = U::from(self.p2_data);
        match p1_data_conv == p2_data_conv {
            true => CycleSeq::Inf(CSeqInf { data: p1_data_conv }),
            false => CycleSeq::LoopLimSin(CycleSeqLoopLimSin {
                p1_data: p1_data_conv,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: p2_data_conv,
            }),
        }
    }
    pub(in crate::svc) fn convert_with_and_optimize<C, U>(self, converter: &mut C) -> CycleSeq<U>
    where
        C: LibConverter<T, U>,
        U: Eq,
    {
        let p1_data_conv = converter.lib_convert(self.p1_data);
        let p2_data_conv = converter.lib_convert(self.p2_data);
        match p1_data_conv == p2_data_conv {
            true => CycleSeq::Inf(CSeqInf { data: p1_data_conv }),
            false => CycleSeq::LoopLimSin(CycleSeqLoopLimSin {
                p1_data: p1_data_conv,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: p2_data_conv,
            }),
        }
    }
}
impl<T> CycleSeqLoopLimSin<T>
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
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::LoopLimSin(*self))
    }
}
impl CycleSeqLoopLimSin<CycleDataDur> {
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
pub(in crate::svc) struct CSeqLoopLimSinCycleIter<T> {
    cseq: CycleSeqLoopLimSin<T>,
    p1_repeats_done: Count,
}
impl<T> CSeqLoopLimSinCycleIter<T> {
    fn new(cseq: CycleSeqLoopLimSin<T>) -> Self {
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
    cseq: &'a CycleSeqLoopLimSin<T>,
    index: usize,
}
impl<'a, T> CSeqLoopLimSinPartIter<'a, T> {
    fn new(cseq: &'a CycleSeqLoopLimSin<T>) -> Self {
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
    cseq: &'a CycleSeqLoopLimSin<T>,
    index: usize,
}
impl<'a, T> CSeqLoopedLoopLimSinPartIter<'a, T> {
    fn new(cseq: &'a CycleSeqLoopLimSin<T>) -> Self {
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
