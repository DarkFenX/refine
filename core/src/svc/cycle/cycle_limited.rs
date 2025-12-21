use crate::{
    def::AttrVal,
    svc::cycle::{
        CycleIterItem,
        cycle_inner_limited::{CycleInnerLimited, CycleInnerLimitedIter},
    },
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimited {
    pub(in crate::svc) inner: CycleInnerLimited,
}
impl CycleLimited {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        match self.inner.charged {
            Some(_) => InfCount::Count(self.inner.repeat_count),
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.get_total_time()
    }
    pub(super) fn iter_cycles(&self) -> CycleLimitedIter {
        CycleLimitedIter::new(self)
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            inner: self.inner.copy_rounded(),
        }
    }
    pub(super) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        self.inner.get_cycle_time_for_stagger()
    }
}

pub(in crate::svc) struct CycleLimitedIter {
    inner: CycleInnerLimitedIter,
}
impl CycleLimitedIter {
    fn new(cycle: &CycleLimited) -> Self {
        Self {
            inner: cycle.inner.iter_cycles(),
        }
    }
}
impl Iterator for CycleLimitedIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
