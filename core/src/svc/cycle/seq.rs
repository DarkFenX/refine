use super::{
    data::CycleHardDt, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin,
};
use crate::util::LibConverter;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeq<D, HDT = CycleHardDt> {
    Lim(CSeqLim<D>),
    Inf(CSeqInf<D, HDT>),
    LimInf(CSeqLimInf<D>),
    LimSinInf(CSeqLimSinInf<D>),
    LoopLimSin(CSeqLoopLimSin<D, HDT>),
}
impl<D, HDT> CycleSeq<D, HDT>
where
    D: Copy,
{
    pub(in crate::svc) fn get_first_cycle(&self) -> &D {
        match self {
            Self::Lim(inner) => inner.get_first_cycle(),
            Self::Inf(inner) => inner.get_first_cycle(),
            Self::LimInf(inner) => inner.get_first_cycle(),
            Self::LimSinInf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<&HDT> {
        match self {
            Self::Lim(_) => None,
            Self::Inf(inner) => inner.get_hard_dt(),
            Self::LimInf(_) => None,
            Self::LimSinInf(_) => None,
            Self::LoopLimSin(inner) => inner.get_hard_dt(),
        }
    }
}

pub(in crate::svc) enum CycleSeqLooped<D, HDT = CycleHardDt> {
    Inf(CSeqInf<D, HDT>),
    LoopLimSin(CSeqLoopLimSin<D, HDT>),
}
impl<D, HDT> CycleSeqLooped<D, HDT>
where
    D: Copy,
{
    pub(in crate::svc) fn get_first_cycle(&self) -> &D {
        match self {
            Self::Inf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<&HDT> {
        match self {
            Self::Inf(inner) => inner.get_hard_dt(),
            Self::LoopLimSin(inner) => inner.get_hard_dt(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeq<D, HDT> {
    pub(in crate::svc) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<D, HDT>>
    where
        D: Copy,
        HDT: Copy,
    {
        match self {
            Self::Lim(inner) => inner.try_loop_cseq(),
            Self::Inf(inner) => inner.try_loop_cseq(),
            Self::LimInf(inner) => inner.try_loop_cseq(),
            Self::LimSinInf(inner) => inner.try_loop_cseq(),
            Self::LoopLimSin(inner) => inner.try_loop_cseq(),
        }
    }
    pub(in crate::svc) fn convert_and_optimize<D2>(self) -> CycleSeq<D2, HDT>
    where
        D2: From<D> + Eq,
    {
        match self {
            Self::Lim(inner) => inner.convert().optimize(),
            Self::Inf(inner) => inner.convert().optimize(),
            Self::LimInf(inner) => inner.convert().optimize(),
            Self::LimSinInf(inner) => inner.convert().optimize(),
            Self::LoopLimSin(inner) => inner.convert().optimize(),
        }
    }
    pub(in crate::svc) fn convert_with_and_optimize<C, D2>(self, converter: &mut C) -> CycleSeq<D2, HDT>
    where
        C: LibConverter<D, D2>,
        D2: Eq,
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

impl<D, HDT> CycleSeqLooped<D, HDT> {
    pub(in crate::svc) fn convert_with_and_optimize<C, D2>(self, converter: &mut C) -> CycleSeqLooped<D2, HDT>
    where
        C: LibConverter<D, D2>,
        D2: Eq,
    {
        match self {
            Self::Inf(inner) => inner.convert_with(converter).optimize_looped(),
            Self::LoopLimSin(inner) => inner.convert_with(converter).optimize_looped(),
        }
    }
}
