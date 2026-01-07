use crate::{
    misc::{Count, InfCount},
    svc::cycle::{
        CycleSeq, CycleSeqLooped,
        seq_inf::{CSeqInfPartIter, CSeqLoopedInfPartIter},
        seq_lim::CSeqLimPartIter,
        seq_lim_inf::CSeqLimInfPartIter,
        seq_lim_sin_inf::CSeqLimSinInfPartIter,
        seq_loop_lim_sin::{CSeqLoopLimSinPartIter, CSeqLoopedLoopLimSinPartIter},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular cycle
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
// Looped cycle
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
