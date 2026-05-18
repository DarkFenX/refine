use super::{seq::CycleSeqLooped, seq_inf::CSeqInf, seq_loop_lim_sin::CSeqLoopLimSin};
use crate::num::Count;

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CycleSeqLooped<D>
where
    D: Copy,
{
    pub(in crate::svc) fn iter_cseq_parts(&self) -> CSeqLoopedPartIter<'_, D> {
        match self {
            Self::Inf(inner) => CSeqLoopedPartIter::Inf(inner.iter_cseq_parts_looped()),
            Self::LoopLimSin(inner) => CSeqLoopedPartIter::LoopLimSin(inner.iter_cseq_parts_looped()),
        }
    }
}

pub(in crate::svc) enum CSeqLoopedPartIter<'a, D> {
    Inf(CSeqLoopedInfPartIter<'a, D>),
    LoopLimSin(CSeqLoopedLoopLimSinPartIter<'a, D>),
}
impl<D> Iterator for CSeqLoopedPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqLoopedPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Inf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

pub(crate) struct CSeqLoopedPart<D> {
    pub(crate) data: D,
    pub(crate) repeat_count: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqInf<D>
where
    D: Copy,
{
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedInfPartIter<'_, D> {
        CSeqLoopedInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopedInfPartIter<'a, D> {
    cseq: &'a CSeqInf<D>,
    yielded: bool,
}
impl<'a, D> CSeqLoopedInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqInf<D>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D> Iterator for CSeqLoopedInfPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqLoopedPart<D>;

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
impl<D> CSeqLoopLimSin<D>
where
    D: Copy,
{
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedLoopLimSinPartIter<'_, D> {
        CSeqLoopedLoopLimSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopedLoopLimSinPartIter<'a, D> {
    cseq: &'a CSeqLoopLimSin<D>,
    index: usize,
}
impl<'a, D> CSeqLoopedLoopLimSinPartIter<'a, D> {
    fn new(cseq: &'a CSeqLoopLimSin<D>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D> Iterator for CSeqLoopedLoopLimSinPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqLoopedPart<D>;

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
