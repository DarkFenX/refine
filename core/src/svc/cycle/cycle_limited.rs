use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleChargedInfo, CycleIterItem, CycleLooped},
    util::{InfCount, sig_round},
};

// Part 1: runs specified number of times
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimited {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
    pub(in crate::svc) repeat_count: Count,
}
impl CycleLimited {
    pub(super) fn get_looped_part(&self) -> Option<CycleLooped> {
        None
    }
    pub(super) fn get_charged_info(&self) -> InfCount {
        match self.charged {
            Some(_) => InfCount::Count(self.repeat_count),
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_cycles(&self) -> CycleLimitedIter {
        CycleLimitedIter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            active_time: sig_round(self.active_time, 10),
            inactive_time: sig_round(self.inactive_time, 10),
            repeat_count: self.repeat_count,
            interrupt: self.interrupt,
            charged: self.charged.map(|v| sig_round(v, 10)),
        }
    }
    pub(super) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
}

pub(in crate::svc) struct CycleLimitedIter {
    item: CycleIterItem,
    repeat_count: Count,
    cycles_done: Count,
}
impl CycleLimitedIter {
    fn new(cycle: &CycleLimited) -> Self {
        Self {
            item: CycleIterItem::new(cycle.active_time + cycle.inactive_time, cycle.interrupt, cycle.charged),
            repeat_count: cycle.repeat_count,
            cycles_done: 0,
        }
    }
}
impl Iterator for CycleLimitedIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.repeat_count {
            return None;
        }
        self.cycles_done += 1;
        Some(self.item)
    }
}
