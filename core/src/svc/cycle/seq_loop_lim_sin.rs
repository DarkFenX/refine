use super::{
    seq::{CycleSeq, CycleSeqLooped},
    seq_inf::CSeqInf,
};
use crate::{num::Count, util::LibConverter};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLoopLimSin<D, HDT> {
    pub(in crate::svc) p1_data: D,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: D,
    // Optional hard downtime every loop
    pub(in crate::svc) hard_dt: Option<HDT>,
}
impl<D, HDT> CSeqLoopLimSin<D, HDT> {
    pub(super) fn get_first_cycle(&self) -> &D {
        &self.p1_data
    }
    pub(super) fn get_hard_dt(&self) -> Option<&HDT> {
        self.hard_dt.as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopLimSin<D, HDT> {
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<D, HDT>>
    where
        D: Copy,
        HDT: Copy,
    {
        Some(CycleSeqLooped::LoopLimSin(*self))
    }
    pub(super) fn convert<D2, HDT2>(self) -> CSeqLoopLimSin<D2, HDT2>
    where
        D2: From<D>,
        HDT2: From<HDT>,
    {
        CSeqLoopLimSin {
            p1_data: D2::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: D2::from(self.p2_data),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn convert_with<C, D2, HDT2>(self, converter: &mut C) -> CSeqLoopLimSin<D2, HDT2>
    where
        C: LibConverter<D, D2>,
        HDT2: From<HDT>,
    {
        CSeqLoopLimSin {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
            hard_dt: self.hard_dt.map(Into::into),
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<D, HDT>
    where
        D: Eq,
    {
        match self.p1_data == self.p2_data && self.hard_dt.is_none() {
            true => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeq::LoopLimSin(self),
        }
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<D, HDT>
    where
        D: Eq,
    {
        match self.p1_data == self.p2_data && self.hard_dt.is_none() {
            true => CycleSeqLooped::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeqLooped::LoopLimSin(self),
        }
    }
}
