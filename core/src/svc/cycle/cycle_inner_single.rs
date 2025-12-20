use crate::{def::AttrVal, svc::cycle::CycleIterItem, util::sig_round};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInnerSingle {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleInnerSingle {
    pub(super) fn get_total_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
    pub(super) fn iter_cycles(&self, force_interrupt: bool) -> CycleInnerSingleIter {
        CycleInnerSingleIter::new(self, force_interrupt)
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

pub(super) struct CycleInnerSingleIter {
    item: CycleIterItem,
    done: bool,
}
impl CycleInnerSingleIter {
    fn new(cycle: &CycleInnerSingle, force_interrupt: bool) -> Self {
        Self {
            item: CycleIterItem::new(
                cycle.active_time + cycle.inactive_time,
                cycle.interrupt || force_interrupt,
                cycle.charged,
            ),
            done: false,
        }
    }
    pub(super) fn reset(&mut self) {
        self.done = false;
    }
}
impl Iterator for CycleInnerSingleIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        self.done = true;
        Some(self.item)
    }
}
