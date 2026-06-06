use super::{
    seq_limited::CycleSeqLimited, seq_looped::CycleSeqLooped, seq_var_lim::CSeqLim, seq_var_lim_sin::CSeqLimSin,
    seq_var_loop_lim_sin::CSeqLoopLimSin, seq_var_loop_sin::CSeqLoopSin,
};
use crate::num::Count;

pub(crate) struct CSeqPart<D> {
    pub(crate) data: D,
    pub(crate) repeat_count: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Looped - high-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeqLooped<D, HDT> {
    pub(in crate::svc) fn iter_parts(&self) -> CSeqLoopedPartIter<'_, D, HDT> {
        match self {
            Self::LoopSin(inner) => CSeqLoopedPartIter::LoopSin(inner.iter_parts_looped()),
            Self::LoopLimSin(inner) => CSeqLoopedPartIter::LoopLimSin(inner.iter_parts_looped()),
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
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::LoopSin(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Looped - LoopSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopSin<D, HDT> {
    fn iter_parts_looped(&self) -> CSeqLoopedLoopSinPartIter<'_, D, HDT> {
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
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CSeqPart {
            data: self.cseq.data,
            repeat_count: Count::ONE,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Looped - LoopLimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopLimSin<D, HDT> {
    fn iter_parts_looped(&self) -> CSeqLoopedLoopLimSinPartIter<'_, D, HDT> {
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
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index = 1;
                Some(CSeqPart {
                    data: self.cseq.p1_data,
                    repeat_count: self.cseq.p1_repeat_count,
                })
            }
            1 => {
                self.index = 2;
                Some(CSeqPart {
                    data: self.cseq.p2_data,
                    repeat_count: Count::ONE,
                })
            }
            2 => None,
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Limited - high-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CycleSeqLimited<D> {
    pub(in crate::svc) fn iter_parts(&self) -> CSeqLimitedPartIter<'_, D> {
        match self {
            Self::Lim(inner) => CSeqLimitedPartIter::Lim(inner.iter_parts_limited()),
            Self::LimSin(inner) => CSeqLimitedPartIter::LimSin(inner.iter_parts_limited()),
        }
    }
}

pub(in crate::svc) enum CSeqLimitedPartIter<'a, D> {
    Lim(CSeqLimitedLimPartIter<'a, D>),
    LimSin(CSeqLimitedLimSinPartIter<'a, D>),
}
impl<D> Iterator for CSeqLimitedPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Lim(inner) => inner.next(),
            Self::LimSin(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Limited - Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLim<D> {
    fn iter_parts_limited(&self) -> CSeqLimitedLimPartIter<'_, D> {
        CSeqLimitedLimPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimitedLimPartIter<'a, D> {
    cseq: &'a CSeqLim<D>,
    yielded: bool,
}
impl<'a, D> CSeqLimitedLimPartIter<'a, D> {
    fn new(cseq: &'a CSeqLim<D>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D> Iterator for CSeqLimitedLimPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CSeqPart {
            data: self.cseq.data,
            repeat_count: self.cseq.repeat_count,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Limited - LimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimSin<D> {
    fn iter_parts_limited(&self) -> CSeqLimitedLimSinPartIter<'_, D> {
        CSeqLimitedLimSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimitedLimSinPartIter<'a, D> {
    cseq: &'a CSeqLimSin<D>,
    index: usize,
}
impl<'a, D> CSeqLimitedLimSinPartIter<'a, D> {
    fn new(cseq: &'a CSeqLimSin<D>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D> Iterator for CSeqLimitedLimSinPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index = 1;
                Some(CSeqPart {
                    data: self.cseq.p1_data,
                    repeat_count: self.cseq.p1_repeat_count,
                })
            }
            1 => {
                self.index = 2;
                Some(CSeqPart {
                    data: self.cseq.p2_data,
                    repeat_count: Count::ONE,
                })
            }
            2 => None,
            _ => unreachable!(),
        }
    }
}
