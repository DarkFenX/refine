use super::cycle_inf::CycleInf;
use crate::{
    AttrVal,
    def::Count,
    svc::cycle::{CycleDataFull, CycleLooped, CyclePart, CyclePartIter},
    util::InfCount,
};

// Part 1: runs specified number of times
// Part 2: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CycleLimInf<T>
where
    T: Copy,
{
    pub(super) fn get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(CycleInf { data: self.p2_data }))
    }
    pub(super) fn get_first(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn iter_parts(&self) -> CyclePartIter<T> {
        CyclePartIter::Two(
            [
                CyclePart {
                    data: self.p1_data,
                    repeat_count: InfCount::Count(self.p1_repeat_count),
                },
                CyclePart {
                    data: self.p2_data,
                    repeat_count: InfCount::Infinite,
                },
            ]
            .into_iter(),
        )
    }
    pub(super) fn iter_events(&self) -> CycleLimInfEventIter<T> {
        CycleLimInfEventIter::new(*self)
    }
}
impl CycleLimInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.p2_data.time
    }
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
        }
    }
}

pub(in crate::svc) struct CycleLimInfEventIter<T> {
    cycle: CycleLimInf<T>,
    index: u8,
    p1_repeats_done: Count,
}
impl<T> CycleLimInfEventIter<T> {
    fn new(cycle: CycleLimInf<T>) -> Self {
        Self {
            cycle,
            index: 0,
            p1_repeats_done: 0,
        }
    }
}
impl<T> Iterator for CycleLimInfEventIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                if self.p1_repeats_done >= self.cycle.p1_repeat_count {
                    self.index = 1;
                    return Some(self.cycle.p2_data);
                }
                self.p1_repeats_done += 1;
                Some(self.cycle.p1_data)
            }
            1 => Some(self.cycle.p2_data),
            _ => unreachable!(),
        }
    }
}
