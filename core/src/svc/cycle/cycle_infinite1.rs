use crate::{
    def::AttrVal,
    svc::cycle::CycleIterItem,
    util::{InfCount, sig_round},
};

// Part 1: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInfinite1 {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleInfinite1 {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        match self.charged {
            Some(_) => InfCount::Infinite,
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_cycles(&self) -> CycleInfinite1Iter {
        CycleInfinite1Iter::new(self)
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
    pub(super) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
}

pub(in crate::svc) struct CycleInfinite1Iter {
    item: CycleIterItem,
}
impl CycleInfinite1Iter {
    fn new(cycle: &CycleInfinite1) -> Self {
        Self {
            item: CycleIterItem::new(cycle.active_time + cycle.inactive_time, cycle.interrupt, cycle.charged),
        }
    }
}
impl Iterator for CycleInfinite1Iter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}
