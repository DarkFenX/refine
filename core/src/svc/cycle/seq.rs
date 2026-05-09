use super::{
    data::CycleDtHard, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin,
};
use crate::util::LibConverter;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeq<T> {
    Lim(CSeqLim<T>),
    Inf(CSeqInf<T>),
    LimInf(CSeqLimInf<T>),
    LimSinInf(CSeqLimSinInf<T>),
    LoopLimSin(CSeqLoopLimSin<T>),
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
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<CycleDtHard> {
        match self {
            Self::Lim(inner) => inner.get_hard_dt(),
            Self::Inf(inner) => inner.get_hard_dt(),
            Self::LimInf(inner) => inner.get_hard_dt(),
            Self::LimSinInf(inner) => inner.get_hard_dt(),
            Self::LoopLimSin(inner) => inner.get_hard_dt(),
        }
    }
}

pub(in crate::svc) enum CycleSeqLooped<T> {
    Inf(CSeqInf<T>),
    LoopLimSin(CSeqLoopLimSin<T>),
}
impl<T> CycleSeqLooped<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_first_cycle(&self) -> &T {
        match self {
            Self::Inf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<CycleDtHard> {
        match self {
            Self::Inf(inner) => inner.get_hard_dt(),
            Self::LoopLimSin(inner) => inner.get_hard_dt(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CycleSeq<T>
where
    T: Copy,
{
    pub(in crate::svc) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        match self {
            Self::Lim(inner) => inner.try_loop_cseq(),
            Self::Inf(inner) => inner.try_loop_cseq(),
            Self::LimInf(inner) => inner.try_loop_cseq(),
            Self::LimSinInf(inner) => inner.try_loop_cseq(),
            Self::LoopLimSin(inner) => inner.try_loop_cseq(),
        }
    }
    pub(in crate::svc) fn convert_and_optimize<U>(self) -> CycleSeq<U>
    where
        U: From<T> + Eq,
    {
        match self {
            Self::Lim(inner) => inner.convert().optimize(),
            Self::Inf(inner) => inner.convert().optimize(),
            Self::LimInf(inner) => inner.convert().optimize(),
            Self::LimSinInf(inner) => inner.convert().optimize(),
            Self::LoopLimSin(inner) => inner.convert().optimize(),
        }
    }
    pub(in crate::svc) fn convert_with_and_optimize<C, U>(self, converter: &mut C) -> CycleSeq<U>
    where
        C: LibConverter<T, U>,
        U: Eq,
    {
        match self {
            Self::Lim(inner) => inner.convert_with(converter).optimize(),
            Self::Inf(inner) => inner.convert_with(converter).optimize(),
            Self::LimInf(inner) => inner.convert_with(converter).optimize(),
            Self::LimSinInf(inner) => inner.convert_with(converter).optimize(),
            Self::LoopLimSin(inner) => inner.convert_with(converter).optimize(),
        }
    }
}

impl<T> CycleSeqLooped<T>
where
    T: Copy,
{
    pub(in crate::svc) fn convert_with_and_optimize<C, U>(self, converter: &mut C) -> CycleSeqLooped<U>
    where
        C: LibConverter<T, U>,
        U: Eq,
    {
        match self {
            Self::Inf(inner) => inner.convert_with(converter).optimize_looped(),
            Self::LoopLimSin(inner) => inner.convert_with(converter).optimize_looped(),
        }
    }
}
