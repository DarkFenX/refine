use super::{
    data::CSeqHardDtFull,
    seq::{CycleSeq, CycleSeqLooped},
};
use crate::util::LibConverter;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqInf<D, HDT> {
    pub(in crate::svc) data: D,
    // Optional hard downtime every cycle
    pub(in crate::svc) hard_dt: Option<HDT>,
}
impl<D, HDT> CSeqInf<D, HDT> {
    pub(super) fn get_first_cycle(&self) -> &D {
        &self.data
    }
    pub(super) fn get_hard_dt(&self) -> Option<&HDT> {
        self.hard_dt.as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqInf<D, HDT> {
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<D, HDT>>
    where
        D: Copy,
        HDT: Copy,
    {
        Some(CycleSeqLooped::Inf(*self))
    }
    pub(super) fn convert<D2, HDT2>(self) -> CSeqInf<D2, HDT2>
    where
        D2: From<D>,
        HDT2: From<HDT>,
    {
        CSeqInf {
            data: self.data.into(),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn convert_with<C, D2, HDT2>(self, converter: &mut C) -> CSeqInf<D2, HDT2>
    where
        C: LibConverter<D, D2>,
        HDT2: From<HDT>,
    {
        CSeqInf {
            data: converter.lib_convert(self.data),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<D, HDT> {
        CycleSeq::Inf(self)
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<D, HDT> {
        CycleSeqLooped::Inf(self)
    }
}
