use super::{
    seq_looped::CycleSeqLooped, seq_var_lim::CSeqLim, seq_var_lim_inf::CSeqLimInf, seq_var_lim_sin_inf::CSeqLimSinInf,
    seq_var_loop_lim_sin::CSeqLoopLimSin, seq_var_loop_sin::CSeqLoopSin,
};
use crate::util::LibConverter;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeq<D, HDT> {
    Lim(CSeqLim<D>),
    LimInf(CSeqLimInf<D>),
    LimSinInf(CSeqLimSinInf<D>),
    LoopSin(CSeqLoopSin<D, HDT>),
    LoopLimSin(CSeqLoopLimSin<D, HDT>),
}
impl<D, HDT> CycleSeq<D, HDT>
where
    D: Copy,
{
    pub(in crate::svc) fn get_first_cycle(&self) -> &D {
        match self {
            Self::Lim(inner) => inner.get_first_cycle(),
            Self::LoopSin(inner) => inner.get_first_cycle(),
            Self::LimInf(inner) => inner.get_first_cycle(),
            Self::LimSinInf(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<&HDT> {
        match self {
            Self::Lim(_) => None,
            Self::LoopSin(inner) => inner.get_hard_dt(),
            Self::LimInf(_) => None,
            Self::LimSinInf(_) => None,
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
            Self::LimInf(inner) => inner.try_loop_cseq(),
            Self::LimSinInf(inner) => inner.try_loop_cseq(),
            Self::LoopSin(inner) => inner.try_loop_cseq(),
            Self::LoopLimSin(inner) => inner.try_loop_cseq(),
        }
    }
    pub(in crate::svc) fn convert_and_optimize<D2, HDT2>(self) -> CycleSeq<D2, HDT2>
    where
        D2: From<D> + Eq,
        HDT2: From<HDT>,
    {
        match self {
            Self::Lim(inner) => inner.convert().optimize(),
            Self::LimInf(inner) => inner.convert().optimize(),
            Self::LimSinInf(inner) => inner.convert().optimize(),
            Self::LoopSin(inner) => inner.convert().optimize(),
            Self::LoopLimSin(inner) => inner.convert().optimize(),
        }
    }
    pub(in crate::svc) fn convert_with_and_optimize<C, D2, HDT2>(self, converter: &mut C) -> CycleSeq<D2, HDT2>
    where
        C: LibConverter<D, D2>,
        D2: Eq,
        HDT2: From<HDT>,
    {
        match self {
            Self::Lim(inner) => inner.convert_with(converter).optimize(),
            Self::LimInf(inner) => inner.convert_with(converter).optimize(),
            Self::LimSinInf(inner) => inner.convert_with(converter).optimize(),
            Self::LoopSin(inner) => inner.convert_with(converter).optimize(),
            Self::LoopLimSin(inner) => inner.convert_with(converter).optimize(),
        }
    }
}
