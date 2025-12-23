use super::{cycle_inf::CycleInf, cycle_lim_inf::CycleLimInf};
use crate::{
    def::{AttrVal, Count},
    svc::cycle::{Cycle, CycleDataFull, CycleDataTime, CycleLooped, CyclePart},
    util::InfCount,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLimSinInf<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    pub(in crate::svc) p3_data: T,
}
impl<T> CycleLimSinInf<T> {
    pub(super) fn get_first(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn convert<'a, U>(&'a self) -> Cycle<U>
    where
        U: From<&'a T> + Eq,
    {
        let p1_data = U::from(&self.p1_data);
        let p2_data = U::from(&self.p2_data);
        let p3_data = U::from(&self.p2_data);
        match (p1_data == p2_data, p2_data == p3_data) {
            // Nothing to merge
            (false, false) => Cycle::LimSinInf(CycleLimSinInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data,
                p3_data,
            }),
            // Merge part 2 into tail
            (false, true) => Cycle::LimInf(CycleLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: p3_data,
            }),
            // Merge part 2 into head
            (true, false) => Cycle::LimInf(CycleLimInf {
                p1_data,
                p1_repeat_count: self.p1_repeat_count + 1,
                p2_data: p3_data,
            }),
            // Whole sequence becomes simple infinity
            (true, true) => Cycle::Inf(CycleInf { data: p1_data }),
        }
    }
}
impl<T> CycleLimSinInf<T>
where
    T: Copy,
{
    pub(super) fn iter_events(&self) -> CycleLimSinInfEventIter<T> {
        CycleLimSinInfEventIter::new(*self)
    }
    pub(super) fn iter_parts_regular(&self) -> CycleLimSinInfPartIter<'_, T> {
        CycleLimSinInfPartIter::new(self)
    }
    pub(super) fn try_get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::Inf(CycleInf { data: self.p3_data }))
    }
}
impl CycleLimSinInf {
    pub(super) fn get_average_time(&self) -> AttrVal {
        self.p3_data.time
    }
}
impl CycleLimSinInf<CycleDataTime> {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
            p3_data: self.p3_data.copy_rounded(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CycleLimSinInfPartIter<'a, T> {
    cycle: &'a CycleLimSinInf<T>,
    index: usize,
}
impl<'a, T> CycleLimSinInfPartIter<'a, T> {
    fn new(cycle: &'a CycleLimSinInf<T>) -> Self {
        Self { cycle, index: 0 }
    }
}
impl<T> Iterator for CycleLimSinInfPartIter<'_, T>
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
                    repeat_count: InfCount::Count(1),
                })
            }
            2 => {
                self.index = 3;
                Some(CyclePart {
                    data: self.cycle.p2_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            3 => None,
            _ => unreachable!(),
        }
    }
}
