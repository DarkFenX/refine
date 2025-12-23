use crate::{
    def::AttrVal,
    svc::cycle::{Cycle, CycleDataFull, CycleDataTime, CycleLooped, CycleLoopedPart, CyclePart},
    util::InfCount,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInf<T = CycleDataFull> {
    pub(in crate::svc) data: T,
}
impl<T> CycleInf<T> {
    pub(super) fn get_first(&self) -> &T {
        &self.data
    }
    pub(super) fn convert<'a, U>(&'a self) -> Cycle<U>
    where
        U: From<&'a T>,
    {
        Cycle::Inf(CycleInf {
            data: (&self.data).into(),
        })
    }
}
impl<T> CycleInf<T>
where
    T: Copy,
{
    pub(super) fn iter_events(&self) -> CycleInfEventIter<T> {
        CycleInfEventIter::new(*self)
    }
    pub(super) fn iter_parts_regular(&self) -> CycleInfPartIter<'_, T> {
        CycleInfPartIter::new(self)
    }
    pub(super) fn iter_parts_looped(&self) -> CycleLoopedInfPartIter<'_, T> {
        CycleLoopedInfPartIter::new(self)
    }
    pub(super) fn try_get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(*self))
    }
}
impl CycleInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.data.time
    }
}
impl CycleInf<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CycleInfEventIter<T> {
    cycle: CycleInf<T>,
}
impl<T> CycleInfEventIter<T> {
    fn new(cycle: CycleInf<T>) -> Self {
        Self { cycle }
    }
}
impl<T> Iterator for CycleInfEventIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.cycle.data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part iterators
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CycleInfPartIter<'a, T> {
    cycle: &'a CycleInf<T>,
    yielded: bool,
}
impl<'a, T> CycleInfPartIter<'a, T> {
    fn new(cycle: &'a CycleInf<T>) -> Self {
        Self { cycle, yielded: false }
    }
}
impl<T> Iterator for CycleInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CyclePart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CyclePart {
            data: self.cycle.data,
            repeat_count: InfCount::Infinite,
        })
    }
}

pub(in crate::svc) struct CycleLoopedInfPartIter<'a, T> {
    cycle: &'a CycleInf<T>,
    yielded: bool,
}
impl<'a, T> CycleLoopedInfPartIter<'a, T> {
    fn new(cycle: &'a CycleInf<T>) -> Self {
        Self { cycle, yielded: false }
    }
}
impl<T> Iterator for CycleLoopedInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CycleLoopedPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CycleLoopedPart {
            data: self.cycle.data,
            repeat_count: 1,
        })
    }
}
