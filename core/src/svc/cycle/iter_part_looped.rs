use super::{seq::CycleSeqLooped, seq_inf::CSeqInf, seq_loop_lim_sin::CSeqLoopLimSin};
use crate::num::Count;

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CycleSeqLooped<T>
where
    T: Copy,
{
    pub(in crate::svc) fn iter_cseq_parts(&self) -> CSeqLoopedPartIter<'_, T> {
        match self {
            Self::Inf(inner) => CSeqLoopedPartIter::Inf(inner.iter_cseq_parts_looped()),
            Self::LoopLimSin(inner) => CSeqLoopedPartIter::LoopLimSin(inner.iter_cseq_parts_looped()),
        }
    }
}

pub(in crate::svc) enum CSeqLoopedPartIter<'a, T> {
    Inf(CSeqLoopedInfPartIter<'a, T>),
    LoopLimSin(CSeqLoopedLoopLimSinPartIter<'a, T>),
}
impl<T> Iterator for CSeqLoopedPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqLoopedPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Inf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

pub(crate) struct CSeqLoopedPart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqInf<T>
where
    T: Copy,
{
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedInfPartIter<'_, T> {
        CSeqLoopedInfPartIter::new(self)
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopLimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLoopLimSin<T>
where
    T: Copy,
{
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedLoopLimSinPartIter<'_, T> {
        CSeqLoopedLoopLimSinPartIter::new(self)
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
