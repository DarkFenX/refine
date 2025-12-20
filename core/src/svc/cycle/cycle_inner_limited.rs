use crate::{
    def::{AttrVal, Count},
    svc::cycle::CycleIterItem,
    util::sig_round,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInnerLimited {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
    pub(in crate::svc) repeat_count: Count,
}
impl CycleInnerLimited {
    pub(super) fn get_total_time(&self) -> AttrVal {
        (self.active_time + self.inactive_time) * self.repeat_count as f64
    }
    pub(super) fn iter_cycles(&self, force_interrupt_last: bool) -> CycleInnerLimitedIter {
        CycleInnerLimitedIter::new(self, force_interrupt_last)
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

pub(super) struct CycleInnerLimitedIter {
    item: CycleIterItem,
    repeat_count: Count,
    force_interrupt_last: bool,
    cycles_done: Count,
}
impl CycleInnerLimitedIter {
    fn new(cycle: &CycleInnerLimited, force_interrupt_last: bool) -> Self {
        Self {
            item: CycleIterItem::new(cycle.active_time + cycle.inactive_time, cycle.interrupt, cycle.charged),
            repeat_count: cycle.repeat_count,
            force_interrupt_last,
            cycles_done: 0,
        }
    }
    pub(super) fn reset(&mut self) {
        self.cycles_done = 0;
    }
}
impl Iterator for CycleInnerLimitedIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cycles_done.cmp(&self.repeat_count) {
            std::cmp::Ordering::Less => {
                self.cycles_done += 1;
                Some(self.item)
            }
            std::cmp::Ordering::Equal => {
                self.cycles_done += 1;
                let mut item = self.item;
                if self.force_interrupt_last {
                    item.interrupt = true;
                }
                Some(item)
            }
            std::cmp::Ordering::Greater => None,
        }
    }
}
