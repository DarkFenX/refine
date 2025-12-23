use super::cycle_inf::CycleInf;
use crate::{
    AttrVal,
    def::Count,
    svc::cycle::{Cycle, CycleDataFull, CycleDataTime, CycleLooped, CyclePart},
    util::InfCount,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CycleLimInf<T> {
    pub(super) fn get_first(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn convert<'a, U>(&'a self) -> Cycle<U>
    where
        U: From<&'a T> + Eq,
    {
        let p1_data = U::from(&self.p1_data);
        let p2_data = U::from(&self.p2_data);
        match p1_data == p2_data {
            true => Cycle::Inf(CycleInf { data: p1_data }),
            false => Cycle::LimInf(CycleLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
            }),
        }
    }
}
impl<T> CycleLimInf<T>
where
    T: Copy,
{
    pub(super) fn iter_events(&self) -> CycleLimInfEventIter<T> {
        CycleLimInfEventIter::new(*self)
    }
    pub(super) fn iter_parts_regular(&self) -> CycleLimInfPartIter<'_, T> {
        CycleLimInfPartIter::new(self)
    }
    pub(super) fn try_get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(CycleInf { data: self.p2_data }))
    }
}
impl CycleLimInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.p2_data.time
    }
}
impl CycleLimInf<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CycleLimInfPartIter<'a, T> {
    cycle: &'a CycleLimInf<T>,
    index: usize,
}
impl<'a, T> CycleLimInfPartIter<'a, T> {
    fn new(cycle: &'a CycleLimInf<T>) -> Self {
        Self { cycle, index: 0 }
    }
}
impl<T> Iterator for CycleLimInfPartIter<'_, T>
where
    T: Copy,
{
    type Item = CyclePart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index = 1;
                Some(CyclePart {
                    data: self.cycle.p1_data,
                    repeat_count: InfCount::Count(self.cycle.p1_repeat_count),
                })
            }
            1 => {
                self.index = 2;
                Some(CyclePart {
                    data: self.cycle.p2_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            2 => None,
            _ => unreachable!(),
        }
    }
}
