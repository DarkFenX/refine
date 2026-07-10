use super::{data::GetMainDuration, seq_var_lim::CSeqLim, seq_var_lim_sin::CSeqLimSin};
use crate::{num::PValue, util::LibConverter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeqLimited<D> {
    Lim(CSeqLim<D>),
    LimSin(CSeqLimSin<D>),
}
impl<D> CycleSeqLimited<D> {
    pub(in crate::svc) fn get_main_duration(&self) -> PValue
    where
        D: GetMainDuration,
    {
        match self {
            Self::Lim(inner) => inner.get_main_duration(),
            Self::LimSin(inner) => inner.get_main_duration(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CycleSeqLimited<D> {
    pub(in crate::svc) fn convert_with_and_optimize<C, D2>(self, converter: &mut C) -> CycleSeqLimited<D2>
    where
        C: LibConverter<D, D2>,
        D2: Eq,
    {
        match self {
            Self::Lim(inner) => inner.convert_with(converter).optimize_limited(),
            Self::LimSin(inner) => inner.convert_with(converter).optimize_limited(),
        }
    }
}
