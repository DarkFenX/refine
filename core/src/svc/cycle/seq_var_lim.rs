use super::{seq::CycleSeq, seq_limited::CycleSeqLimited, seq_split::CycleSeqSplit};
use crate::{num::Count, util::LibConverter};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLim<D> {
    pub(in crate::svc) data: D,
    pub(in crate::svc) repeat_count: Count,
}
impl<D> CSeqLim<D> {
    pub(super) fn get_first_cycle(&self) -> &D {
        &self.data
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLim<D> {
    pub(super) fn split_lim_loop<HDT>(self) -> CycleSeqSplit<D, HDT> {
        CycleSeqSplit {
            limited: Some(CycleSeqLimited::Lim(self)),
            looped: None,
        }
    }
    pub(super) fn convert<D2>(self) -> CSeqLim<D2>
    where
        D2: From<D>,
    {
        CSeqLim {
            data: self.data.into(),
            repeat_count: self.repeat_count,
        }
    }
    pub(in crate::svc) fn convert_with<C, D2>(self, converter: &mut C) -> CSeqLim<D2>
    where
        C: LibConverter<D, D2>,
    {
        CSeqLim {
            data: converter.lib_convert(self.data),
            repeat_count: self.repeat_count,
        }
    }
    pub(in crate::svc) fn optimize<HDT>(self) -> CycleSeq<D, HDT> {
        CycleSeq::Lim(self)
    }
}
