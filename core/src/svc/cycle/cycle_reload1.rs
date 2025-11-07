use super::cycle_shared::{CycleInner, CycleInnerIter};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload1 {
    pub(in crate::svc) inner: CycleInner,
}
impl CycleReload1 {
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        InfCount::Count(self.inner.repeat_count)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.active_time + self.inner.inactive_time
    }
    pub(super) fn iter_cycles(&self) -> CycleReload1Iter {
        CycleReload1Iter::new(self)
    }
}

pub(in crate::svc) struct CycleReload1Iter {
    inner: CycleInnerIter,
}
impl CycleReload1Iter {
    fn new(cycle: &CycleReload1) -> Self {
        Self {
            inner: cycle.inner.iter_cycles(),
        }
    }
}
impl Iterator for CycleReload1Iter {
    type Item = AttrVal;

    fn next(&mut self) -> Option<Self::Item> {
        // Reload implies that we can loop over child iterator infinitely
        match self.inner.next() {
            Some(time) => Some(time),
            // Even if inner iterator first response is None, worst-case we will get the same after
            // the reset
            None => {
                self.inner.reset();
                self.inner.next()
            }
        }
    }
}
