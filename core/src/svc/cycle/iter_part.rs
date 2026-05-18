use super::{
    seq::CycleSeq, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin,
};
use crate::{misc::InfCount, num::Count};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CycleSeq<D>
where
    D: Copy,
{
    pub(in crate::svc) fn get_cseq_parts(&self) -> CSeqParts<'_, D> {
        let loops = match self {
            Self::Lim(_) | Self::Inf(_) | Self::LimInf(_) | Self::LimSinInf(_) => false,
            Self::LoopLimSin(_) => true,
        };
        CSeqParts { cseq: self, loops }
    }
}

pub(crate) struct CSeqParts<'a, D> {
    cseq: &'a CycleSeq<D>,
    pub(crate) loops: bool,
}
impl<'a, D> CSeqParts<'a, D>
where
    D: Copy,
{
    pub(crate) fn iter(&self) -> CSeqPartIter<'a, D> {
        match self.cseq {
            CycleSeq::Lim(inner) => CSeqPartIter::Lim(inner.iter_cseq_parts_regular()),
            CycleSeq::Inf(inner) => CSeqPartIter::Inf(inner.iter_cseq_parts_regular()),
            CycleSeq::LimInf(inner) => CSeqPartIter::LimInf(inner.iter_cseq_parts_regular()),
            CycleSeq::LimSinInf(inner) => CSeqPartIter::LimSinInf(inner.iter_cseq_parts_regular()),
            CycleSeq::LoopLimSin(inner) => CSeqPartIter::LoopLimSin(inner.iter_cseq_parts_regular()),
        }
    }
}

pub(in crate::svc) enum CSeqPartIter<'a, D> {
    Lim(CSeqLimPartIter<'a, D>),
    Inf(CSeqInfPartIter<'a, D>),
    LimInf(CSeqLimInfPartIter<'a, D>),
    LimSinInf(CSeqLimSinInfPartIter<'a, D>),
    LoopLimSin(CSeqLoopLimSinPartIter<'a, D>),
}
impl<D> Iterator for CSeqPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPart<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Lim(inner) => inner.next(),
            Self::Inf(inner) => inner.next(),
            Self::LimInf(inner) => inner.next(),
            Self::LimSinInf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

pub(crate) struct CSeqPart<D> {
    pub(crate) data: D,
    pub(crate) repeat_count: InfCount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLim<D>
where
    D: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimPartIter<'_, D> {
        CSeqLimPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimPartIter<'a, D> {
    cseq: &'a CSeqLim<D>,
    yielded: bool,
}
impl<'a, D> CSeqLimPartIter<'a, D> {
    fn new(cseq: &'a CSeqLim<D>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D> Iterator for CSeqLimPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPart<D>;

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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqInf<D>
where
    D: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqInfPartIter<'_, D> {
        CSeqInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqInfPartIter<'a, D> {
    cseq: &'a CSeqInf<D>,
    yielded: bool,
}
impl<'a, D> CSeqInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqInf<D>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D> Iterator for CSeqInfPartIter<'_, D>
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
            repeat_count: InfCount::Infinite,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimInf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimInf<D>
where
    D: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimInfPartIter<'_, D> {
        CSeqLimInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimInfPartIter<'a, D> {
    cseq: &'a CSeqLimInf<D>,
    index: usize,
}
impl<'a, D> CSeqLimInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqLimInf<D>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D> Iterator for CSeqLimInfPartIter<'_, D>
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimSinInf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimSinInf<D>
where
    D: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimSinInfPartIter<'_, D> {
        CSeqLimSinInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimSinInfPartIter<'a, D> {
    cseq: &'a CSeqLimSinInf<D>,
    index: usize,
}
impl<'a, D> CSeqLimSinInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqLimSinInf<D>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D> Iterator for CSeqLimSinInfPartIter<'_, D>
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
                    data: self.cseq.p3_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            3 => None,
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopLimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLoopLimSin<D>
where
    D: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLoopLimSinPartIter<'_, D> {
        CSeqLoopLimSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopLimSinPartIter<'a, D> {
    cseq: &'a CSeqLoopLimSin<D>,
    index: usize,
}
impl<'a, D> CSeqLoopLimSinPartIter<'a, D> {
    fn new(cseq: &'a CSeqLoopLimSin<D>) -> Self {
        Self { cseq, index: 0 }
    }
}
impl<D> Iterator for CSeqLoopLimSinPartIter<'_, D>
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
