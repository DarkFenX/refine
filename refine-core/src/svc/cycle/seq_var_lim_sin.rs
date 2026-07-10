use super::{data::GetMainDuration, seq_limited::CycleSeqLimited, seq_var_lim::CSeqLim};
use crate::{
    num::{Count, PValue},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: runs once
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimSin<D> {
    pub(in crate::svc) p1_data: D,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: D,
}
impl<D> CSeqLimSin<D> {
    pub(super) fn get_main_duration(&self) -> PValue
    where
        D: GetMainDuration,
    {
        self.p1_data
            .get_main_duration()
            .mul_add(self.p1_repeat_count.into_pvalue(), self.p2_data.get_main_duration())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimSin<D> {
    pub(in crate::svc) fn convert_with<C, D2>(self, converter: &mut C) -> CSeqLimSin<D2>
    where
        C: LibConverter<D, D2>,
    {
        CSeqLimSin {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
        }
    }
    pub(super) fn optimize_limited(self) -> CycleSeqLimited<D>
    where
        D: Eq,
    {
        match self.p1_data == self.p2_data {
            true => CycleSeqLimited::Lim(CSeqLim {
                data: self.p1_data,
                repeat_count: self.p1_repeat_count + Count::ONE,
            }),
            false => CycleSeqLimited::LimSin(self),
        }
    }
}
