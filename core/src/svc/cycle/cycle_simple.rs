use super::cycle_shared::time_round;
use crate::{
    def::{AttrVal, Count},
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleSimple {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: InfCount,
}
impl CycleSimple {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            active_time: time_round(self.active_time),
            inactive_time: time_round(self.inactive_time),
            repeat_count: self.repeat_count,
        }
    }
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        self.repeat_count
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_cycles(&self) -> CycleSimpleIter {
        CycleSimpleIter::new(self)
    }
}

pub(in crate::svc) struct CycleSimpleIter {
    total_cycle_time: AttrVal,
    repeat_count: InfCount,
    cycles_done: Count,
}
impl CycleSimpleIter {
    fn new(cycle: &CycleSimple) -> Self {
        Self {
            total_cycle_time: cycle.active_time + cycle.inactive_time,
            repeat_count: cycle.repeat_count,
            cycles_done: 0,
        }
    }
}
impl Iterator for CycleSimpleIter {
    type Item = AttrVal;

    fn next(&mut self) -> Option<Self::Item> {
        match self.repeat_count {
            InfCount::Count(count) => match self.cycles_done >= count {
                true => None,
                false => {
                    self.cycles_done += 1;
                    Some(self.total_cycle_time)
                }
            },
            InfCount::Infinite => Some(self.total_cycle_time),
        }
    }
}
