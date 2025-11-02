use crate::def::{AttrVal, Count};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleInner {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: Count,
}
impl CycleInner {
    pub(super) fn get_total_time(&self) -> AttrVal {
        (self.active_time + self.inactive_time) * self.repeat_count as f64
    }
}
