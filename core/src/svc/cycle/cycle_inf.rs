use crate::{
    def::AttrVal,
    svc::cycle::{CycleChargedInfo, CycleChargedInfoIter, CycleEventItem, CycleLooped},
    util::{InfCount, sig_round},
};

// Part 1: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInf {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleInf {
    pub(super) fn get_looped_part(&self) -> Option<CycleLooped> {
        Some(CycleLooped::Inf(*self))
    }
    pub(super) fn iter_charged_info(&self) -> CycleChargedInfoIter {
        CycleChargedInfoIter::One(
            [CycleChargedInfo {
                repeat_count: InfCount::Infinite,
                charged: self.charged,
            }]
            .into_iter(),
        )
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_events(&self) -> CycleInfEventIter {
        CycleInfEventIter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            active_time: sig_round(self.active_time, 10),
            inactive_time: sig_round(self.inactive_time, 10),
            interrupt: self.interrupt,
            charged: self.charged.map(|v| sig_round(v, 10)),
        }
    }
    pub(super) fn get_first_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
}

pub(in crate::svc) struct CycleInfEventIter {
    item: CycleEventItem,
}
impl CycleInfEventIter {
    fn new(cycle: &CycleInf) -> Self {
        Self {
            item: CycleEventItem::new(cycle.active_time + cycle.inactive_time, cycle.interrupt, cycle.charged),
        }
    }
}
impl Iterator for CycleInfEventIter {
    type Item = CycleEventItem;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}
