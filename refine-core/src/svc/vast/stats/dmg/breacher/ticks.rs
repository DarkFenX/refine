use crate::num::Count;

// Has [start, end) meaning in context of breacher calculations
#[derive(Copy, Clone)]
pub(super) struct TickRange {
    pub(super) start: Count,
    pub(super) end: Count,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AggrBreacherTicks {
    Infinite(AbtInfinite),
    CycleSimple(AbtCycleSimple),
    CycleComplex(AbtCycleComplex),
}
impl AggrBreacherTicks {
    pub(super) fn get_initial_delay(&self) -> Count {
        match self {
            Self::Infinite(inner) => inner.get_initial_delay(),
            Self::CycleSimple(inner) => inner.get_initial_delay(),
            Self::CycleComplex(inner) => inner.get_initial_delay(),
        }
    }
    pub(super) fn get_loop_len(&self) -> Count {
        match self {
            Self::Infinite(inner) => inner.get_loop_len(),
            Self::CycleSimple(inner) => inner.get_loop_len(),
            Self::CycleComplex(inner) => inner.get_loop_len(),
        }
    }
    // Within requested range, find damage ranges (ranges where damage occurs every tick), and call
    // passed function for each of them
    pub(super) fn call_for_dmg_ranges<F>(&self, req_range: TickRange, func: &mut F)
    where
        F: FnMut(TickRange),
    {
        match self {
            Self::Infinite(inner) => inner.call_for_dmg_ranges(req_range, func),
            Self::CycleSimple(inner) => inner.call_for_dmg_ranges(req_range, func),
            Self::CycleComplex(inner) => inner.call_for_dmg_ranges(req_range, func),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle: infinite damage ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtInfinite {
    pub(super) initial_delay: Count,
}
impl AbtInfinite {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        Count::ONE
    }
    fn call_for_dmg_ranges<F>(&self, req_range: TickRange, func: &mut F)
    where
        F: FnMut(TickRange),
    {
        call_for_overlap_range(
            TickRange {
                start: self.initial_delay,
                end: req_range.end,
            },
            req_range,
            func,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle:
// - damage ticks
// - gap ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtCycleSimple {
    pub(super) initial_delay: Count,
    pub(super) cycle_dmg: Count,
    pub(super) cycle_total: Count,
}
impl AbtCycleSimple {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        self.cycle_total
    }
    fn call_for_dmg_ranges<F>(&self, req_range: TickRange, func: &mut F)
    where
        F: FnMut(TickRange),
    {
        let mut cycle_start = get_first_cycle_start(self.initial_delay, self.cycle_total, req_range.start);
        while cycle_start < req_range.end {
            call_for_overlap_range(
                TickRange {
                    start: cycle_start,
                    end: cycle_start + self.cycle_dmg,
                },
                req_range,
                func,
            );
            cycle_start += self.cycle_total;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle:
// - repeated N times:
//   - damage ticks
//   - gap ticks
// - damage ticks
// - gap ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtCycleComplex {
    pub(super) initial_delay: Count,
    pub(super) cycle_p1_dmg: Count,
    pub(super) cycle_p1_total: Count,
    pub(super) cycle_p1_repeats: Count,
    pub(super) cycle_p2_dmg: Count,
    pub(super) cycle_p2_total: Count,
}
impl AbtCycleComplex {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        self.cycle_p1_total * self.cycle_p1_repeats + self.cycle_p2_total
    }
    fn call_for_dmg_ranges<F>(&self, req_range: TickRange, func: &mut F)
    where
        F: FnMut(TickRange),
    {
        let p1_total_ticks = self.cycle_p1_total * self.cycle_p1_repeats;
        let loop_total_ticks = p1_total_ticks + self.cycle_p2_total;
        let mut loop_start = get_first_cycle_start(self.initial_delay, loop_total_ticks, req_range.start);
        while loop_start < req_range.end {
            // Part 1 - limited
            if self.cycle_p1_total > Count::ZERO && self.cycle_p1_dmg > Count::ZERO {
                let mut cycle_start = loop_start;
                let mut repeats_left = self.cycle_p1_repeats;
                while repeats_left > Count::ZERO && cycle_start < req_range.end {
                    call_for_overlap_range(
                        TickRange {
                            start: cycle_start,
                            end: cycle_start + self.cycle_p1_dmg,
                        },
                        req_range,
                        func,
                    );
                    cycle_start += self.cycle_p1_total;
                    repeats_left -= Count::ONE;
                }
            }
            // Part 2 - single
            if self.cycle_p2_dmg > Count::ZERO {
                let p2_start = loop_start + p1_total_ticks;
                call_for_overlap_range(
                    TickRange {
                        start: p2_start,
                        end: p2_start + self.cycle_p2_dmg,
                    },
                    req_range,
                    func,
                );
            }
            loop_start += loop_total_ticks;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dmg range helper funcs
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_first_cycle_start(initial_delay: Count, cycle_total: Count, range_start: Count) -> Count {
    match range_start > initial_delay {
        true => initial_delay + (range_start - initial_delay) / cycle_total * cycle_total,
        false => initial_delay,
    }
}

fn call_for_overlap_range<F>(range1: TickRange, range2: TickRange, func: &mut F)
where
    F: FnMut(TickRange),
{
    let start = range1.start.max(range2.start);
    let end = range1.end.min(range2.end);
    if start < end {
        func(TickRange { start, end });
    }
}
