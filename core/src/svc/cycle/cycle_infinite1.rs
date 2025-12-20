use crate::{
    def::AttrVal,
    svc::cycle::{
        CycleIterItem,
        cycle_inner_infinite::{CycleInnerInfinite, CycleInnerInfiniteIter},
    },
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInfinite1 {
    pub(in crate::svc) inner: CycleInnerInfinite,
}
impl CycleInfinite1 {
    pub(super) fn get_charged_cycles(&self) -> InfCount {
        match self.inner.charged {
            Some(_) => InfCount::Infinite,
            None => InfCount::Count(0),
        }
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.get_total_time()
    }
    pub(super) fn iter_cycles(&self) -> CycleInfinite1Iter {
        CycleInfinite1Iter::new(self)
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

pub(in crate::svc) struct CycleInfinite1Iter {
    inner: CycleInnerInfiniteIter,
}
impl CycleInfinite1Iter {
    fn new(cycle: &CycleInfinite1) -> Self {
        Self {
            inner: cycle.inner.iter_cycles(),
        }
    }
}
impl Iterator for CycleInfinite1Iter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
