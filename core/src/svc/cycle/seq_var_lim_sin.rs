use super::data::GetMainDuration;
use crate::num::{Count, PValue};

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
