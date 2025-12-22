use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleChargedInfo, CycleChargedInfoIter, CycleIterItem, CycleLooped},
    util::{InfCount, sig_round},
};

// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLoopLimSin {
    pub(in crate::svc) p1_active_time: AttrVal,
    pub(in crate::svc) p1_inactive_time: AttrVal,
    pub(in crate::svc) p1_interrupt: bool,
    pub(in crate::svc) p1_charged: Option<AttrVal>,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_active_time: AttrVal,
    pub(in crate::svc) p2_inactive_time: AttrVal,
    pub(in crate::svc) p2_interrupt: bool,
    pub(in crate::svc) p2_charged: Option<AttrVal>,
}
impl CycleLoopLimSin {
    pub(super) fn get_looped_part(&self) -> Option<CycleLooped> {
        Some(CycleLooped::LoopLimSin(*self))
    }
    pub(super) fn get_charged_info(&self) -> CycleChargedInfoIter {
        CycleChargedInfoIter::Two(
            [
                CycleChargedInfo {
                    repeat_count: InfCount::Count(self.p1_repeat_count),
                    charged: self.p1_charged,
                },
                CycleChargedInfo {
                    repeat_count: InfCount::Count(1),
                    charged: self.p2_charged,
                },
            ]
            .into_iter(),
        )
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        let p1_total_time = (self.p1_active_time + self.p1_inactive_time) * self.p1_repeat_count as f64;
        let p2_total_time = self.p2_active_time + self.p2_inactive_time;
        (p1_total_time + p2_total_time) / (self.p1_repeat_count + 1) as f64
    }
    pub(super) fn iter_cycles(&self) -> CycleLoopLimSinIter {
        CycleLoopLimSinIter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_active_time: sig_round(self.p1_active_time, 10),
            p1_inactive_time: sig_round(self.p1_inactive_time, 10),
            p1_repeat_count: self.p1_repeat_count,
            p1_interrupt: self.p1_interrupt,
            p1_charged: self.p1_charged.map(|v| sig_round(v, 10)),
            p2_active_time: sig_round(self.p2_active_time, 10),
            p2_inactive_time: sig_round(self.p2_inactive_time, 10),
            p2_interrupt: self.p2_interrupt,
            p2_charged: self.p2_charged.map(|v| sig_round(v, 10)),
        }
    }
    pub(super) fn get_first_cycle_time(&self) -> AttrVal {
        self.p1_active_time + self.p1_inactive_time
    }
}

pub(in crate::svc) struct CycleLoopLimSinIter {
    p1_item: CycleIterItem,
    p1_repeat_count: Count,
    p1_cycles_done: Count,
    p2_item: CycleIterItem,
}
impl CycleLoopLimSinIter {
    fn new(cycle: &CycleLoopLimSin) -> Self {
        Self {
            p1_item: CycleIterItem::new(
                cycle.p1_active_time + cycle.p1_inactive_time,
                cycle.p1_interrupt,
                cycle.p1_charged,
            ),
            p1_repeat_count: cycle.p1_repeat_count,
            p1_cycles_done: 0,
            p2_item: CycleIterItem::new(
                cycle.p2_active_time + cycle.p2_inactive_time,
                cycle.p2_interrupt,
                cycle.p2_charged,
            ),
        }
    }
}
impl Iterator for CycleLoopLimSinIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p1_cycles_done >= self.p1_repeat_count {
            self.p1_cycles_done = 0;
            return Some(self.p2_item);
        }
        self.p1_cycles_done += 1;
        Some(self.p1_item)
    }
}
