use super::cycle_inf::CycleInf;
use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleChargedInfo, CycleChargedInfoIter, CycleEventItem, CycleLooped},
    util::{InfCount, sig_round},
};

// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimSinInf {
    pub(in crate::svc) p1_active_time: AttrVal,
    pub(in crate::svc) p1_inactive_time: AttrVal,
    pub(in crate::svc) p1_interrupt: bool,
    pub(in crate::svc) p1_charged: Option<AttrVal>,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_active_time: AttrVal,
    pub(in crate::svc) p2_inactive_time: AttrVal,
    pub(in crate::svc) p2_interrupt: bool,
    pub(in crate::svc) p2_charged: Option<AttrVal>,
    pub(in crate::svc) p3_active_time: AttrVal,
    pub(in crate::svc) p3_inactive_time: AttrVal,
    pub(in crate::svc) p3_interrupt: bool,
    pub(in crate::svc) p3_charged: Option<AttrVal>,
}
impl CycleLimSinInf {
    pub(super) fn get_looped_part(&self) -> Option<CycleLooped> {
        Some(CycleLooped::Inf(CycleInf {
            active_time: self.p3_active_time,
            inactive_time: self.p3_inactive_time,
            interrupt: self.p3_interrupt,
            charged: self.p3_charged,
        }))
    }
    pub(super) fn get_charged_info(&self) -> CycleChargedInfoIter {
        CycleChargedInfoIter::Three(
            [
                CycleChargedInfo {
                    repeat_count: InfCount::Count(self.p1_repeat_count),
                    charged: self.p1_charged,
                },
                CycleChargedInfo {
                    repeat_count: InfCount::Count(1),
                    charged: self.p2_charged,
                },
                CycleChargedInfo {
                    repeat_count: InfCount::Infinite,
                    charged: self.p3_charged,
                },
            ]
            .into_iter(),
        )
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.p1_active_time + self.p1_inactive_time
    }
    pub(super) fn iter_events(&self) -> CycleLimSinInfEventIter {
        CycleLimSinInfEventIter::new(self)
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
            p3_active_time: sig_round(self.p3_active_time, 10),
            p3_inactive_time: sig_round(self.p3_inactive_time, 10),
            p3_interrupt: self.p3_interrupt,
            p3_charged: self.p3_charged.map(|v| sig_round(v, 10)),
        }
    }
    pub(super) fn get_first_cycle_time(&self) -> AttrVal {
        self.p1_active_time + self.p1_inactive_time
    }
}

pub(in crate::svc) struct CycleLimSinInfEventIter {
    index: u8,
    p1_item: CycleEventItem,
    p1_repeat_count: Count,
    p1_cycles_done: Count,
    p2_item: CycleEventItem,
    p3_item: CycleEventItem,
}
impl CycleLimSinInfEventIter {
    fn new(cycle: &CycleLimSinInf) -> Self {
        Self {
            index: 0,
            p1_item: CycleEventItem::new(
                cycle.p1_active_time + cycle.p1_inactive_time,
                cycle.p1_interrupt,
                cycle.p1_charged,
            ),
            p1_repeat_count: cycle.p1_repeat_count,
            p1_cycles_done: 0,
            p2_item: CycleEventItem::new(
                cycle.p2_active_time + cycle.p2_inactive_time,
                cycle.p2_interrupt,
                cycle.p2_charged,
            ),
            p3_item: CycleEventItem::new(
                cycle.p3_active_time + cycle.p3_inactive_time,
                cycle.p3_interrupt,
                cycle.p3_charged,
            ),
        }
    }
}
impl Iterator for CycleLimSinInfEventIter {
    type Item = CycleEventItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                if self.p1_cycles_done >= self.p1_repeat_count {
                    self.index = 1;
                    return Some(self.p2_item);
                }
                self.p1_cycles_done += 1;
                Some(self.p1_item)
            }
            1 => Some(self.p3_item),
            _ => unreachable!(),
        }
    }
}
