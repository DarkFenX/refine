use crate::{
    def::AttrVal,
    svc::cycle::{
        CycleIterItem,
        cycle_inner_limited::{CycleInnerLimited, CycleInnerLimitedIter},
        cycle_inner_single::{CycleInnerSingle, CycleInnerSingleIter},
    },
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleReload2 {
    pub(in crate::svc) inner1: CycleInnerLimited,
    pub(in crate::svc) inner2: CycleInnerSingle,
}
impl CycleReload2 {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        InfCount::Count(self.inner1.repeat_count + 1)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        (self.inner1.get_total_time() + self.inner2.get_total_time()) / (self.inner1.repeat_count + 1) as f64
    }
    pub(super) fn iter_cycles(&self) -> CycleReload2Iter {
        CycleReload2Iter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            inner1: self.inner1.copy_rounded(),
            inner2: self.inner2.copy_rounded(),
        }
    }
    pub(super) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        self.inner1.get_cycle_time_for_stagger()
    }
}

pub(in crate::svc) struct CycleReload2Iter {
    inner1: CycleInnerLimitedIter,
    inner2: CycleInnerSingleIter,
    index: u8,
    yielded: bool,
}
impl CycleReload2Iter {
    fn new(cycle: &CycleReload2) -> Self {
        Self {
            inner1: cycle.inner1.iter_cycles(),
            inner2: cycle.inner2.iter_cycles(),
            index: 0,
            yielded: false,
        }
    }
}
impl Iterator for CycleReload2Iter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => match self.inner1.next() {
                Some(item) => {
                    self.yielded = true;
                    Some(item)
                }
                None => {
                    self.index = 1;
                    self.next()
                }
            },
            1 => match self.inner2.next() {
                Some(item) => {
                    self.yielded = true;
                    Some(item)
                }
                None => match self.yielded {
                    true => {
                        self.index = 0;
                        self.inner1.reset();
                        self.inner2.reset();
                        self.next()
                    }
                    false => {
                        self.index = 2;
                        self.next()
                    }
                },
            },
            2 => None,
            _ => unreachable!(),
        }
    }
}
