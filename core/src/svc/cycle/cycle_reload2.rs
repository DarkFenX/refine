use super::cycle_shared::{CycleInner, CycleInnerIter};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload2 {
    pub(in crate::svc) inner_early: CycleInner,
    pub(in crate::svc) inner_final: CycleInner,
}
impl CycleReload2 {
    pub(super) fn get_cycles_until_empty(&self) -> InfCount {
        InfCount::Count(self.inner_early.repeat_count + self.inner_final.repeat_count)
    }
    pub(super) fn get_average_cycle_time(&self) -> AttrVal {
        (self.inner_early.get_total_time() + self.inner_final.get_total_time())
            / (self.inner_early.repeat_count + self.inner_final.repeat_count) as f64
    }
    pub(super) fn iter_cycles(&self) -> CycleReload2Iter {
        CycleReload2Iter::new(self)
    }
}

pub(in crate::svc) struct CycleReload2Iter {
    inners: [CycleInnerIter; 2],
    iter_index: usize,
    yielded: bool,
}
impl CycleReload2Iter {
    fn new(cycle: &CycleReload2) -> Self {
        Self {
            inners: [cycle.inner_early.iter_cycles(), cycle.inner_final.iter_cycles()],
            iter_index: 0,
            yielded: false,
        }
    }
}
impl Iterator for CycleReload2Iter {
    type Item = AttrVal;

    fn next(&mut self) -> Option<Self::Item> {
        // Reload implies that we can loop over child iterators infinitely
        let iter = &mut self.inners[self.iter_index];
        match iter.next() {
            Some(time) => {
                self.yielded = true;
                Some(time)
            }
            None => match self.iter_index {
                // First iterator is always unconditionally switched to second
                0 => {
                    iter.reset();
                    self.iter_index = 1;
                    self.next()
                }
                // Second is switched back to first only if either of them yielded anything
                1 if self.yielded => {
                    iter.reset();
                    self.iter_index = 0;
                    self.next()
                }
                _ => None,
            },
        }
    }
}
