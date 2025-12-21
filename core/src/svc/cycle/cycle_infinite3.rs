use crate::{
    def::AttrVal,
    svc::cycle::{
        CycleIterItem,
        cycle_inner_infinite::{CycleInnerInfinite, CycleInnerInfiniteIter},
        cycle_inner_limited::{CycleInnerLimited, CycleInnerLimitedIter},
        cycle_inner_single::{CycleInnerSingle, CycleInnerSingleIter},
    },
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInfinite3 {
    pub(in crate::svc) inner1: CycleInnerLimited,
    pub(in crate::svc) inner2: CycleInnerSingle,
    pub(in crate::svc) inner3: CycleInnerInfinite,
}
impl CycleInfinite3 {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        if self.inner3.charged.is_some() {
            return InfCount::Infinite;
        }
        let mut cycles = match self.inner1.charged {
            Some(_) => self.inner1.repeat_count,
            None => 0,
        };
        if self.inner2.charged.is_some() {
            cycles += 1;
        }
        InfCount::Count(cycles)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner3.get_total_time()
    }
    pub(super) fn iter_cycles(&self) -> CycleInfinite3Iter {
        CycleInfinite3Iter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            inner1: self.inner1.copy_rounded(),
            inner2: self.inner2.copy_rounded(),
            inner3: self.inner3.copy_rounded(),
        }
    }
    pub(super) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        self.inner1.get_cycle_time_for_stagger()
    }
}

pub(in crate::svc) struct CycleInfinite3Iter {
    inner1: CycleInnerLimitedIter,
    inner2: CycleInnerSingleIter,
    inner3: CycleInnerInfiniteIter,
    index: u8,
}
impl CycleInfinite3Iter {
    fn new(cycle: &CycleInfinite3) -> Self {
        Self {
            inner1: cycle.inner1.iter_cycles(),
            inner2: cycle.inner2.iter_cycles(),
            inner3: cycle.inner3.iter_cycles(),
            index: 0,
        }
    }
}
impl Iterator for CycleInfinite3Iter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => match self.inner1.next() {
                Some(item) => Some(item),
                None => {
                    self.index = 1;
                    self.next()
                }
            },
            1 => match self.inner2.next() {
                Some(item) => Some(item),
                None => {
                    self.index = 2;
                    self.next()
                }
            },
            2 => self.inner3.next(),
            _ => unreachable!(),
        }
    }
}
