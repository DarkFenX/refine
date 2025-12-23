use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleDataFull, CycleLooped, CyclePart, CyclePartIter},
    util::InfCount,
};

// Part 1: runs specified number of times
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLim<T = CycleDataFull> {
    pub(in crate::svc) data: T,
    pub(in crate::svc) repeat_count: Count,
}
impl<T> CycleLim<T>
where
    T: Copy,
{
    pub(super) fn get_loop(&self) -> Option<CycleLooped<T>> {
        None
    }
    pub(super) fn get_first(&self) -> &T {
        &self.data
    }
    pub(super) fn iter_parts(&self) -> CyclePartIter<T> {
        CyclePartIter::One(
            [CyclePart {
                data: self.data,
                repeat_count: InfCount::Count(self.repeat_count),
            }]
            .into_iter(),
        )
    }
    pub(super) fn iter_events(&self) -> CycleLimEventIter<T> {
        CycleLimEventIter::new(*self)
    }
}
impl CycleLim {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.data.time
    }
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
            repeat_count: self.repeat_count,
        }
    }
}

pub(in crate::svc) struct CycleLimEventIter<T> {
    cycle: CycleLim<T>,
    repeats_done: Count,
}
impl<T> CycleLimEventIter<T> {
    fn new(cycle: CycleLim<T>) -> Self {
        Self { cycle, repeats_done: 0 }
    }
}
impl<T> Iterator for CycleLimEventIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeats_done >= self.cycle.repeat_count {
            return None;
        }
        self.repeats_done += 1;
        Some(self.cycle.data)
    }
}
