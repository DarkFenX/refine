use crate::{
    def::{AttrVal, Count},
    svc::cycle::{Cycle, CycleDataFull, CycleDataTime, CycleLooped, CyclePart},
    util::InfCount,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLim<T = CycleDataFull> {
    pub(in crate::svc) data: T,
    pub(in crate::svc) repeat_count: Count,
}
impl<T> CycleLim<T> {
    pub(super) fn get_first(&self) -> &T {
        &self.data
    }
    pub(super) fn try_get_loop(&self) -> Option<CycleLooped<T>> {
        None
    }
    pub(super) fn convert<'a, U>(&'a self) -> Cycle<U>
    where
        U: From<&'a T>,
    {
        Cycle::Lim(CycleLim {
            data: (&self.data).into(),
            repeat_count: self.repeat_count,
        })
    }
}
impl<T> CycleLim<T>
where
    T: Copy,
{
    pub(super) fn iter_events(&self) -> CycleLimEventIter<T> {
        CycleLimEventIter::new(*self)
    }
    pub(super) fn iter_parts_regular(&self) -> CycleLimPartIter<'_, T> {
        CycleLimPartIter::new(self)
    }
}
impl CycleLim {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.data.time
    }
}
impl CycleLim<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
            repeat_count: self.repeat_count,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part data + iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CycleLimPartIter<'a, T> {
    cycle: &'a CycleLim<T>,
    yielded: bool,
}
impl<'a, T> CycleLimPartIter<'a, T> {
    fn new(cycle: &'a CycleLim<T>) -> Self {
        Self { cycle, yielded: false }
    }
}
impl<T> Iterator for CycleLimPartIter<'_, T>
where
    T: Copy,
{
    type Item = CyclePart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        Some(CyclePart {
            data: self.cycle.data,
            repeat_count: InfCount::Count(self.cycle.repeat_count),
        })
    }
}
