use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleSimple {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: InfCount,
}
impl CycleSimple {
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        self.repeat_count
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
}
