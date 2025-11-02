use super::cycle_shared::CycleInner;
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload2 {
    pub(in crate::svc) inner_early: CycleInner,
    pub(in crate::svc) inner_final: CycleInner,
}
impl CycleReload2 {
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        InfCount::Count(self.inner_early.repeat_count + self.inner_final.repeat_count)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        (self.inner_early.get_total_time() + self.inner_final.get_total_time())
            / (self.inner_early.repeat_count + self.inner_final.repeat_count) as f64
    }
    pub(super) fn iter_cycles(&self) -> impl Iterator<Item = AttrVal> {
        std::iter::chain(self.inner_early.iter_cycles(), self.inner_final.iter_cycles())
    }
}
