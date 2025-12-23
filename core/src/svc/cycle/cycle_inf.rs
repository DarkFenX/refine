use crate::{
    def::AttrVal,
    svc::cycle::{Cycle, CycleDataFull, CycleDataTime, CycleLooped, CyclePart},
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
    pub(super) fn iter_parts(&self) -> CycleInfPartIter<'_, T> {
        CycleInfPartIter::new(self)
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
// Part iterator
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
        Some(CyclePart {
            data: self.cycle.data,
            repeat_count: InfCount::Infinite,
        })
    }
}
