use super::cycle_shared::CycleInner;
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload1 {
    pub(in crate::svc) inner: CycleInner,
}
impl CycleReload1 {
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        InfCount::Count(self.inner.repeat_count)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.active_time + self.inner.inactive_time
    }
}
