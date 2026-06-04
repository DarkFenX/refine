use super::{seq_enum::CycleSeq, seq_enum_looped::CycleSeqLooped, seq_lim_inf::CSeqLimInf, seq_loop_sin::CSeqLoopSin};
use crate::{num::Count, util::LibConverter};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimSinInf<D> {
    pub(in crate::svc) p1_data: D,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: D,
    pub(in crate::svc) p3_data: D,
}
impl<D> CSeqLimSinInf<D> {
    pub(super) fn get_first_cycle(&self) -> &D {
        &self.p1_data
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimSinInf<D> {
    pub(super) fn try_loop_cseq<HDT>(&self) -> Option<CycleSeqLooped<D, HDT>>
    where
        D: Copy,
    {
        Some(CycleSeqLooped::LoopSin(CSeqLoopSin {
            data: self.p3_data,
            hard_dt: None,
        }))
    }
    pub(super) fn convert<D2>(self) -> CSeqLimSinInf<D2>
    where
        D2: From<D>,
    {
        CSeqLimSinInf {
            p1_data: D2::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: D2::from(self.p2_data),
            p3_data: D2::from(self.p3_data),
        }
    }
    pub(in crate::svc) fn convert_with<C, D2>(self, converter: &mut C) -> CSeqLimSinInf<D2>
    where
        C: LibConverter<D, D2>,
    {
        CSeqLimSinInf {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
            p3_data: converter.lib_convert(self.p3_data),
        }
    }
    pub(in crate::svc) fn optimize<HDT>(self) -> CycleSeq<D, HDT>
    where
        D: Eq,
    {
        match (self.p1_data == self.p2_data, self.p2_data == self.p3_data) {
            // Nothing to merge
            (false, false) => CycleSeq::LimSinInf(CSeqLimSinInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: self.p2_data,
                p3_data: self.p3_data,
            }),
            // Merge part 2 into tail
            (false, true) => CycleSeq::LimInf(CSeqLimInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: self.p3_data,
            }),
            // Merge part 2 into head
            (true, false) => CycleSeq::LimInf(CSeqLimInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count + Count::ONE,
                p2_data: self.p3_data,
            }),
            // Whole sequence becomes a simple infinity
            (true, true) => CycleSeq::LoopSin(CSeqLoopSin {
                data: self.p1_data,
                hard_dt: None,
            }),
        }
    }
}
