use crate::{
    def::AttrVal,
    svc::cycle::{CycleDataFull, CycleLooped, CyclePart, CyclePartIter},
    util::InfCount,
};

// Part 1: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleInf<T = CycleDataFull> {
    pub(in crate::svc) data: T,
}
impl<T> CycleInf<T>
where
    T: Copy,
{
    pub(super) fn get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(*self))
    }
    pub(super) fn get_first(&self) -> &T {
        &self.data
    }
    pub(super) fn iter_parts(&self) -> CyclePartIter<T> {
        CyclePartIter::One(
            [CyclePart {
                data: self.data,
                repeat_count: InfCount::Infinite,
            }]
            .into_iter(),
        )
    }
    pub(super) fn iter_events(&self) -> CycleInfEventIter<T> {
        CycleInfEventIter::new(*self)
    }
}
impl CycleInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.data.time
    }
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            data: self.data.copy_rounded(),
        }
    }
}

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
