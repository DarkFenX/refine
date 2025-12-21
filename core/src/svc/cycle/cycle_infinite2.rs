use crate::{
    def::AttrVal,
    svc::cycle::{
        CycleIterItem,
        cycle_inner_infinite::{CycleInnerInfinite, CycleInnerInfiniteIter},
        cycle_inner_limited::{CycleInnerLimited, CycleInnerLimitedIter},
    },
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInfinite2 {
    pub(in crate::svc) inner1: CycleInnerLimited,
    pub(in crate::svc) inner2: CycleInnerInfinite,
}
impl CycleInfinite2 {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        if self.inner2.charged.is_some() {
            return InfCount::Infinite;
        }
        match self.inner1.charged {
            Some(_) => InfCount::Count(self.inner1.repeat_count),
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner2.get_total_time()
    }
    pub(super) fn iter_cycles(&self) -> CycleInfinite2Iter {
        CycleInfinite2Iter::new(self)
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

pub(in crate::svc) struct CycleInfinite2Iter {
    inner1: CycleInnerLimitedIter,
    inner2: CycleInnerInfiniteIter,
    index: u8,
}
impl CycleInfinite2Iter {
    fn new(cycle: &CycleInfinite2) -> Self {
        Self {
            inner1: cycle.inner1.iter_cycles(),
            inner2: cycle.inner2.iter_cycles(),
            index: 0,
        }
    }
}
impl Iterator for CycleInfinite2Iter {
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
            1 => self.inner2.next(),
            _ => unreachable!(),
        }
    }
}
