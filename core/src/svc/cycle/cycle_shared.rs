use crate::def::{AttrVal, Count};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInner {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: Count,
}
impl CycleInner {
    pub(super) fn get_total_time(&self) -> AttrVal {
        (self.active_time + self.inactive_time) * self.repeat_count as f64
    }
    pub(super) fn iter_cycles(&self) -> CycleInnerIter {
        CycleInnerIter::new(self)
    }
}

pub(super) struct CycleInnerIter {
    total_cycle_time: AttrVal,
    repeat_count: Count,
    cycles_done: Count,
}
impl CycleInnerIter {
    fn new(cycle: &CycleInner) -> Self {
        Self {
            total_cycle_time: cycle.active_time + cycle.inactive_time,
            repeat_count: cycle.repeat_count,
            cycles_done: 0,
        }
    }
    pub(super) fn reset(&mut self) {
        self.cycles_done = 0;
    }
}
impl Iterator for CycleInnerIter {
    type Item = AttrVal;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cycles_done >= self.repeat_count {
            true => None,
            false => {
                self.cycles_done += 1;
                Some(self.total_cycle_time)
            }
        }
    }
}
