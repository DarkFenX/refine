use super::{
    seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CycleSeqLoopLimSin,
};
use crate::{
    def::AttrVal,
    svc::cycle::{CycleDataFull, CycleDataTime, CycleDataTimeCharge},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeq<T = CycleDataFull> {
    Lim(CSeqLim<T>),
    Inf(CSeqInf<T>),
    LimInf(CSeqLimInf<T>),
    LimSinInf(CSeqLimSinInf<T>),
    LoopLimSin(CycleSeqLoopLimSin<T>),
}
impl<T> CycleSeq<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_first_cycle(&self) -> &T {
        match self {
            Self::Lim(inner) => inner.get_first_cycle(),
            Self::Inf(inner) => inner.get_first_cycle(),
            Self::LimInf(inner) => inner.get_first_cycle(),
            Self::LimSinInf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        match self {
            Self::Lim(inner) => inner.try_loop_cseq(),
            Self::Inf(inner) => inner.try_loop_cseq(),
            Self::LimInf(inner) => inner.try_loop_cseq(),
            Self::LimSinInf(inner) => inner.try_loop_cseq(),
            Self::LoopLimSin(inner) => inner.try_loop_cseq(),
        }
    }
}
impl CycleSeq {
    pub(in crate::svc) fn get_average_time(&self) -> AttrVal {
        match self {
            Self::Lim(inner) => inner.get_time(),
            Self::Inf(inner) => inner.get_time(),
            Self::LimInf(inner) => inner.get_average_time(),
            Self::LimSinInf(inner) => inner.get_average_time(),
            Self::LoopLimSin(inner) => inner.get_average_time(),
        }
    }
    // Convenience conversion methods, to avoid type hinting in some cases
    pub(in crate::svc) fn to_time_charge(&self) -> CycleSeq<CycleDataTimeCharge> {
        self.into()
    }
    pub(in crate::svc) fn to_time(&self) -> CycleSeq<CycleDataTime> {
        self.into()
    }
}
impl CycleSeq<CycleDataTime> {
    pub(in crate::svc) fn copy_rounded(&self) -> Self {
        match self {
            Self::Lim(inner) => Self::Lim(inner.copy_rounded()),
            Self::Inf(inner) => Self::Inf(inner.copy_rounded()),
            Self::LimInf(inner) => Self::LimInf(inner.copy_rounded()),
            Self::LimSinInf(inner) => Self::LimSinInf(inner.copy_rounded()),
            Self::LoopLimSin(inner) => Self::LoopLimSin(inner.copy_rounded()),
        }
    }
}
impl<T, U> From<&CycleSeq<T>> for CycleSeq<U>
where
    U: Eq + for<'a> From<&'a T>,
{
    fn from(cseq: &CycleSeq<T>) -> Self {
        match cseq {
            CycleSeq::Lim(inner) => inner.convert(),
            CycleSeq::Inf(inner) => inner.convert(),
            CycleSeq::LimInf(inner) => inner.convert(),
            CycleSeq::LimSinInf(inner) => inner.convert(),
            CycleSeq::LoopLimSin(inner) => inner.convert(),
        }
    }
}

pub(in crate::svc) enum CycleSeqLooped<T = CycleDataFull> {
    Inf(CSeqInf<T>),
    LoopLimSin(CycleSeqLoopLimSin<T>),
}
impl<T> CycleSeqLooped<T> {
    pub(in crate::svc) fn get_first_cycle(&self) -> &T {
        match self {
            Self::Inf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
}
impl CycleSeqLooped {
    // TODO: consider if it is correct to use it, or if looped parts should be used everywhere
    pub(in crate::svc) fn get_average_time(&self) -> AttrVal {
        match self {
            Self::Inf(inner) => inner.get_time(),
            Self::LoopLimSin(inner) => inner.get_average_time(),
        }
    }
}
