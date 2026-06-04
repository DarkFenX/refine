use super::{seq::CycleSeq, seq_looped::CycleSeqLooped, seq_split::CycleSeqSplit};
use crate::util::LibConverter;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Following parts are lopped:
// Part 1: runs once
// Optional hard downtime
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLoopSin<D, HDT> {
    pub(in crate::svc) data: D,
    // Optional hard downtime every cycle
    pub(in crate::svc) hard_dt: Option<HDT>,
}
impl<D, HDT> CSeqLoopSin<D, HDT> {
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
impl<D, HDT> CSeqLoopSin<D, HDT> {
    pub(super) fn split_lim_loop(self) -> CycleSeqSplit<D, HDT> {
        CycleSeqSplit {
            limited: None,
            looped: Some(CycleSeqLooped::LoopSin(self)),
        }
    }
    pub(super) fn convert<D2, HDT2>(self) -> CSeqLoopSin<D2, HDT2>
    where
        D2: From<D>,
        HDT2: From<HDT>,
    {
        CSeqLoopSin {
            data: self.data.into(),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn convert_with<C, D2, HDT2>(self, converter: &mut C) -> CSeqLoopSin<D2, HDT2>
    where
        C: LibConverter<D, D2>,
        HDT2: From<HDT>,
    {
        CSeqLoopSin {
            data: converter.lib_convert(self.data),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<D, HDT> {
        CycleSeq::LoopSin(self)
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<D, HDT> {
        CycleSeqLooped::LoopSin(self)
    }
}
