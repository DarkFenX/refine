use super::{
    seq::CycleSeq, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin,
};
use crate::{misc::InfCount, num::Count};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CycleSeq<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_cseq_parts(&self) -> CSeqParts<'_, T> {
        let loops = match self {
            Self::Lim(_) | Self::Inf(_) | Self::LimInf(_) | Self::LimSinInf(_) => false,
            Self::LoopLimSin(_) => true,
        };
        CSeqParts { cseq: self, loops }
    }
}

pub(crate) struct CSeqParts<'a, T> {
    cseq: &'a CycleSeq<T>,
    pub(crate) loops: bool,
}
impl<'a, T> CSeqParts<'a, T>
where
    T: Copy,
{
    pub(crate) fn iter(&self) -> CSeqPartIter<'a, T> {
        match self.cseq {
            CycleSeq::Lim(inner) => CSeqPartIter::Lim(inner.iter_cseq_parts_regular()),
            CycleSeq::Inf(inner) => CSeqPartIter::Inf(inner.iter_cseq_parts_regular()),
            CycleSeq::LimInf(inner) => CSeqPartIter::LimInf(inner.iter_cseq_parts_regular()),
            CycleSeq::LimSinInf(inner) => CSeqPartIter::LimSinInf(inner.iter_cseq_parts_regular()),
            CycleSeq::LoopLimSin(inner) => CSeqPartIter::LoopLimSin(inner.iter_cseq_parts_regular()),
        }
    }
}

pub(in crate::svc) enum CSeqPartIter<'a, T> {
    Lim(CSeqLimPartIter<'a, T>),
    Inf(CSeqInfPartIter<'a, T>),
    LimInf(CSeqLimInfPartIter<'a, T>),
    LimSinInf(CSeqLimSinInfPartIter<'a, T>),
    LoopLimSin(CSeqLoopLimSinPartIter<'a, T>),
}
impl<T> Iterator for CSeqPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

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

pub(crate) struct CSeqPart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: InfCount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLim<T>
where
    T: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimPartIter<'_, T> {
        CSeqLimPartIter::new(self)
    }
}

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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqInf<T>
where
    T: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqInfPartIter<'_, T> {
        CSeqInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqInfPartIter<'a, T> {
    cseq: &'a CSeqInf<T>,
    yielded: bool,
}
impl<'a, T> CSeqInfPartIter<'a, T> {
    fn new(cseq: &'a CSeqInf<T>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<T> Iterator for CSeqInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

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
impl<T> CSeqLimInf<T>
where
    T: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimInfPartIter<'_, T> {
        CSeqLimInfPartIter::new(self)
    }
}

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

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimSinInf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLimSinInf<T>
where
    T: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLimSinInfPartIter<'_, T> {
        CSeqLimSinInfPartIter::new(self)
    }
}

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
impl<T> CSeqLoopLimSin<T>
where
    T: Copy,
{
    fn iter_cseq_parts_regular(&self) -> CSeqLoopLimSinPartIter<'_, T> {
        CSeqLoopLimSinPartIter::new(self)
    }
}

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
