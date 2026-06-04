use super::{seq_enum_looped::CycleSeqLooped, seq_loop_lim_sin::CSeqLoopLimSin, seq_loop_sin::CSeqLoopSin};
use crate::num::Count;

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeqLooped<D, HDT> {
    pub(in crate::svc) fn iter_cseq_parts(&self) -> CSeqLoopedPartIter<'_, D, HDT> {
        match self {
            Self::LoopSin(inner) => CSeqLoopedPartIter::LoopSin(inner.iter_cseq_parts_looped()),
            Self::LoopLimSin(inner) => CSeqLoopedPartIter::LoopLimSin(inner.iter_cseq_parts_looped()),
        }
    }
}

pub(in crate::svc) enum CSeqLoopedPartIter<'a, D, HDT> {
    LoopSin(CSeqLoopedLoopSinPartIter<'a, D, HDT>),
    LoopLimSin(CSeqLoopedLoopLimSinPartIter<'a, D, HDT>),
}
impl<D, HDT> Iterator for CSeqLoopedPartIter<'_, D, HDT>
where
    D: Copy,
{
    type Item = CSeqLoopedPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::LoopSin(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

pub(crate) struct CSeqLoopedPart<D> {
    pub(crate) data: D,
    pub(crate) repeat_count: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopSin<D, HDT> {
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedLoopSinPartIter<'_, D, HDT> {
        CSeqLoopedLoopSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopedLoopSinPartIter<'a, D, HDT> {
    cseq: &'a CSeqLoopSin<D, HDT>,
    yielded: bool,
}
impl<'a, D, HDT> CSeqLoopedLoopSinPartIter<'a, D, HDT> {
    fn new(cseq: &'a CSeqLoopSin<D, HDT>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D, HDT> Iterator for CSeqLoopedLoopSinPartIter<'_, D, HDT>
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
impl<D, HDT> CSeqLoopLimSin<D, HDT> {
    fn iter_cseq_parts_looped(&self) -> CSeqLoopedLoopLimSinPartIter<'_, D, HDT> {
        CSeqLoopedLoopLimSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopedLoopLimSinPartIter<'a, D, HDT> {
    cseq: &'a CSeqLoopLimSin<D, HDT>,
    index: usize,
}
impl<'a, D, HDT> CSeqLoopedLoopLimSinPartIter<'a, D, HDT> {
    fn new(cseq: &'a CSeqLoopLimSin<D, HDT>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D, HDT> Iterator for CSeqLoopedLoopLimSinPartIter<'_, D, HDT>
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
