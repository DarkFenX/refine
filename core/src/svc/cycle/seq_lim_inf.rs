use super::{
    seq::{CycleSeq, CycleSeqLooped},
    seq_inf::CSeqInf,
};
use crate::{num::Count, util::LibConverter};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimInf<D> {
    pub(in crate::svc) p1_data: D,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: D,
}
impl<D> CSeqLimInf<D> {
    pub(super) fn get_first_cycle(&self) -> &D {
        &self.p1_data
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimInf<D> {
    pub(super) fn convert<D2>(self) -> CSeqLimInf<D2>
    where
        D2: From<D>,
    {
        CSeqLimInf {
            p1_data: D2::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: D2::from(self.p2_data),
        }
    }
    pub(in crate::svc) fn convert_with<C, D2>(self, converter: &mut C) -> CSeqLimInf<D2>
    where
        C: LibConverter<D, D2>,
    {
        CSeqLimInf {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
        }
    }
    pub(in crate::svc) fn optimize<HDT>(self) -> CycleSeq<D, HDT>
    where
        D: Eq,
    {
        match self.p1_data == self.p2_data {
            true => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeq::LimInf(self),
        }
    }
}
impl<D> CSeqLimInf<D>
where
    D: Copy,
{
    pub(super) fn try_loop_cseq<HDT>(&self) -> Option<CycleSeqLooped<D, HDT>> {
        Some(CycleSeqLooped::Inf(CSeqInf {
            data: self.p2_data,
            hard_dt: None,
        }))
    }
}
