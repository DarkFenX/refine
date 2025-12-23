use super::cycle_inf::CycleInf;
use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleDataFull, CycleLooped, CyclePart, CyclePartIter},
    util::InfCount,
};

// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimSinInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    pub(in crate::svc) p3_data: T,
}
impl<T> CycleLimSinInf<T>
where
    T: Copy,
{
    pub(super) fn get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(CycleInf { data: self.p3_data }))
    }
    pub(super) fn get_first(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn iter_parts(&self) -> CyclePartIter<T> {
        CyclePartIter::Three(
            [
                CyclePart {
                    data: self.p1_data,
                    repeat_count: InfCount::Count(self.p1_repeat_count),
                },
                CyclePart {
                    data: self.p2_data,
                    repeat_count: InfCount::Count(1),
                },
                CyclePart {
                    data: self.p3_data,
                    repeat_count: InfCount::Infinite,
                },
            ]
            .into_iter(),
        )
    }
    pub(super) fn iter_events(&self) -> CycleLimSinInfEventIter<T> {
        CycleLimSinInfEventIter::new(*self)
    }
}
impl CycleLimSinInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.p3_data.time
    }
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
            p3_data: self.p3_data.copy_rounded(),
        }
    }
}

pub(in crate::svc) struct CycleLimSinInfEventIter<T> {
    cycle: CycleLimSinInf<T>,
    index: u8,
    p1_repeats_done: Count,
}
impl<T> CycleLimSinInfEventIter<T> {
    fn new(cycle: CycleLimSinInf<T>) -> Self {
        Self {
            cycle,
            index: 0,
            p1_repeats_done: 0,
        }
    }
}
impl<T> Iterator for CycleLimSinInfEventIter<T>
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
            1 => Some(self.cycle.p3_data),
            _ => unreachable!(),
        }
    }
}
