use crate::{
    def::{AttrVal, Count},
    svc::cycle::{CycleDataFull, CycleLooped, CyclePart, CyclePartIter},
    util::InfCount,
};

// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleLoopLimSin<T = CycleDataFull> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CycleLoopLimSin<T>
where
    T: Copy,
{
    pub(super) fn get_loop(&self) -> Option<CycleLooped<T>> {
        Some(CycleLooped::LoopLimSin(*self))
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
                    repeat_count: InfCount::Count(1),
                },
            ]
            .into_iter(),
        )
    }
    pub(super) fn iter_events(&self) -> CycleLoopLimSinEventIter<T> {
        CycleLoopLimSinEventIter::new(*self)
    }
}
impl CycleLoopLimSin {
    pub(super) fn get_average_time(&self) -> AttrVal {
        let p1_total_time = self.p1_data.time * self.p1_repeat_count as f64;
        let p2_total_time = self.p2_data.time;
        (p1_total_time + p2_total_time) / (self.p1_repeat_count + 1) as f64
    }
    // Methods used in cycle staggering
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            p1_data: self.p1_data.copy_rounded(),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: self.p2_data.copy_rounded(),
        }
    }
}

pub(in crate::svc) struct CycleLoopLimSinEventIter<T> {
    cycle: CycleLoopLimSin<T>,
    p1_repeats_done: Count,
}
impl<T> CycleLoopLimSinEventIter<T> {
    fn new(cycle: CycleLoopLimSin<T>) -> Self {
        Self {
            cycle,
            p1_repeats_done: 0,
        }
    }
}
impl<T> Iterator for CycleLoopLimSinEventIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p1_repeats_done >= self.cycle.p1_repeat_count {
            self.p1_repeats_done = 0;
            return Some(self.cycle.p2_data);
        }
        self.p1_repeats_done += 1;
        Some(self.cycle.p1_data)
    }
}
