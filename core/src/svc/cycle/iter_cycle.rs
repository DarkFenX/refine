use crate::svc::cycle::{
    CycleSeq, seq_inf::CSeqInfCycleIter, seq_lim::CSeqLimCycleIter, seq_lim_inf::CSeqLimInfCycleIter,
    seq_lim_sin_inf::CSeqLimSinInfCycleIter, seq_loop_lim_sin::CSeqLoopLimSinCycleIter,
};

impl<T> CycleSeq<T>
where
    T: Copy,
{
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter<T> {
        match self {
            Self::Lim(inner) => CycleIter::Lim(inner.iter_cycles()),
            Self::Inf(inner) => CycleIter::Inf(inner.iter_cycles()),
            Self::LimInf(inner) => CycleIter::LimInf(inner.iter_cycles()),
            Self::LimSinInf(inner) => CycleIter::LimSinInf(inner.iter_cycles()),
            Self::LoopLimSin(inner) => CycleIter::LoopLimSin(inner.iter_cycles()),
        }
    }
}

pub(in crate::svc) enum CycleIter<T> {
    Lim(CSeqLimCycleIter<T>),
    Inf(CSeqInfCycleIter<T>),
    LimInf(CSeqLimInfCycleIter<T>),
    LimSinInf(CSeqLimSinInfCycleIter<T>),
    LoopLimSin(CSeqLoopLimSinCycleIter<T>),
}
impl<T> Iterator for CycleIter<T>
where
    T: Copy,
{
    type Item = T;

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
