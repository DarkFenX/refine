use super::{seq_var_loop_lim_sin::CSeqLoopLimSin, seq_var_loop_sin::CSeqLoopSin};
use crate::util::LibConverter;

pub(in crate::svc) enum CycleSeqLooped<D, HDT> {
    LoopSin(CSeqLoopSin<D, HDT>),
    LoopLimSin(CSeqLoopLimSin<D, HDT>),
}
impl<D, HDT> CycleSeqLooped<D, HDT> {
    pub(in crate::svc) fn get_first_cycle(&self) -> &D {
        match self {
            Self::LoopSin(inner) => inner.get_first_cycle(),
            Self::LoopLimSin(inner) => inner.get_first_cycle(),
        }
    }
    pub(in crate::svc) fn get_hard_dt(&self) -> Option<&HDT> {
        match self {
            Self::LoopSin(inner) => inner.get_hard_dt(),
            Self::LoopLimSin(inner) => inner.get_hard_dt(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeqLooped<D, HDT> {
    pub(in crate::svc) fn convert_with_and_optimize<C, D2, HDT2>(self, converter: &mut C) -> CycleSeqLooped<D2, HDT2>
    where
        C: LibConverter<D, D2>,
        D2: Eq,
        HDT2: From<HDT>,
    {
        match self {
            Self::LoopSin(inner) => inner.convert_with(converter).optimize_looped(),
            Self::LoopLimSin(inner) => inner.convert_with(converter).optimize_looped(),
        }
    }
}
