use crate::{
    def::AttrVal,
    svc::cycle::{CycleChargedInfo, CycleIterItem, CycleLooped},
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
    pub(super) fn get_charged_info(&self) -> InfCount {
        match self.charged {
            Some(_) => InfCount::Infinite,
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_cycles(&self) -> CycleInfIter {
        CycleInfIter::new(self)
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

pub(in crate::svc) struct CycleInfIter {
    item: CycleIterItem,
}
impl CycleInfIter {
    fn new(cycle: &CycleInf) -> Self {
        Self {
            item: CycleIterItem::new(cycle.active_time + cycle.inactive_time, cycle.interrupt, cycle.charged),
        }
    }
}
impl Iterator for CycleInfIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}
