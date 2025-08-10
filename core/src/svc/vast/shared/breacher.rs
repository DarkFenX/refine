use crate::{
    def::{AttrVal, Count, SERVER_TICK_S},
    svc::{cycle::Cycle, output::OutputDmgBreacher, vast::StatDmgBreacher},
    util::{InfCount, RMap, ceil_unerr},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacher {
    absolute_max: AttrVal,
    relative_max: AttrVal,
    ticks: AggrBreacherTicks,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum AggrBreacherTicks {
    Simple(InfCount),
    Complex1(AggrBreacherTicksComplex1),
    Complex2(AggrBreacherTicksComplex2),
}
impl AggrBreacherTicks {
    fn is_infinite(&self) -> bool {
        match &self {
            Self::Simple(count) => matches!(count, InfCount::Infinite),
            Self::Complex1(complex1) => matches!(complex1.repeat_count, InfCount::Infinite),
            // Complex2 is implicitly infinitely cycling
            Self::Complex2(_) => true,
        }
    }
    fn get_cycle_ticks(&self) -> Count {
        match &self {
            Self::Simple(_) => 1,
            Self::Complex1(complex1) => complex1.get_cycle_ticks(),
            Self::Complex2(complex2) => complex2.get_cycle_ticks(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacherTicksComplex1 {
    dmg_tick_count: Count,
    inactive_tick_count: Count,
    repeat_count: InfCount,
}
impl AggrBreacherTicksComplex1 {
    fn get_cycle_ticks(&self) -> Count {
        self.dmg_tick_count + self.inactive_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacherTicksInner {
    dmg_tick_count: Count,
    inactive_tick_count: Count,
    repeat_count: Count,
}
impl AggrBreacherTicksInner {
    fn get_cycle_ticks(&self) -> Count {
        self.dmg_tick_count + self.inactive_tick_count
    }
}

// This one is implied to repeat infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacherTicksComplex2 {
    inner_early: AggrBreacherTicksInner,
    inner_final: AggrBreacherTicksInner,
}
impl AggrBreacherTicksComplex2 {
    fn get_cycle_ticks(&self) -> Count {
        self.inner_early.get_cycle_ticks() + self.inner_final.get_cycle_ticks()
    }
}

// Process breacher module cycle data + output per cycle into some kind of aggregated value, which
// discards all overlapping instances and aligns everything to ticks, which is needed for further
// processing
fn aggregate_breacher(opc: OutputDmgBreacher, cycle: Cycle) -> Option<AggrBreacher> {
    match cycle {
        Cycle::Simple(simple) => {
            if let InfCount::Count(0) = simple.repeat_count {
                return None;
            }
            let full_cycle_ticks = ceil_unerr((simple.active_time + simple.inactive_time) / SERVER_TICK_S) as Count;
            match opc.tick_count >= full_cycle_ticks {
                // Case when breacher damage is enough to reach beginning of next cycle
                true => match simple.repeat_count {
                    InfCount::Count(repeats) => Some(AggrBreacher {
                        absolute_max: opc.absolute_max,
                        relative_max: opc.relative_max,
                        // All instances applied during all cycles but final + whatever last
                        // breacher applies
                        ticks: AggrBreacherTicks::Simple(InfCount::Count(
                            full_cycle_ticks * (repeats - 1) + opc.tick_count,
                        )),
                    }),
                    // Unlimited cycles - breacher is applied continuously and permanently
                    InfCount::Infinite => Some(AggrBreacher {
                        absolute_max: opc.absolute_max,
                        relative_max: opc.relative_max,
                        ticks: AggrBreacherTicks::Simple(InfCount::Infinite),
                    }),
                },
                // Case when there are the same gaps every cycle
                false => Some(AggrBreacher {
                    absolute_max: opc.absolute_max,
                    relative_max: opc.relative_max,
                    ticks: AggrBreacherTicks::Complex1(AggrBreacherTicksComplex1 {
                        dmg_tick_count: opc.tick_count,
                        inactive_tick_count: full_cycle_ticks - opc.tick_count,
                        repeat_count: simple.repeat_count,
                    }),
                }),
            }
        }
        Cycle::Reload1(reload1) => {
            let full_cycle_ticks =
                ceil_unerr((reload1.inner.active_time + reload1.inner.inactive_time) / SERVER_TICK_S) as Count;
            match opc.tick_count >= full_cycle_ticks {
                // Case when breacher damage is enough to reach beginning of next cycle
                true => Some(AggrBreacher {
                    absolute_max: opc.absolute_max,
                    relative_max: opc.relative_max,
                    ticks: AggrBreacherTicks::Simple(InfCount::Infinite),
                }),
                // Case when there are the same gaps every cycle
                false => Some(AggrBreacher {
                    absolute_max: opc.absolute_max,
                    relative_max: opc.relative_max,
                    ticks: AggrBreacherTicks::Complex1(AggrBreacherTicksComplex1 {
                        dmg_tick_count: opc.tick_count,
                        inactive_tick_count: full_cycle_ticks - opc.tick_count,
                        repeat_count: InfCount::Infinite,
                    }),
                }),
            }
        }
        Cycle::Reload2(reload2) => {
            let early_full_cycle_ticks =
                ceil_unerr((reload2.inner_early.active_time + reload2.inner_early.inactive_time) / SERVER_TICK_S)
                    as Count;
            let final_full_cycle_ticks =
                ceil_unerr((reload2.inner_final.active_time + reload2.inner_final.inactive_time) / SERVER_TICK_S)
                    as Count;
            if opc.tick_count >= early_full_cycle_ticks {
                // Breacher duration covers all possible gaps
                if opc.tick_count >= final_full_cycle_ticks {
                    return Some(AggrBreacher {
                        absolute_max: opc.absolute_max,
                        relative_max: opc.relative_max,
                        ticks: AggrBreacherTicks::Simple(InfCount::Infinite),
                    });
                }
                if reload2.inner_final.repeat_count == 1 {
                    return Some(AggrBreacher {
                        absolute_max: opc.absolute_max,
                        relative_max: opc.relative_max,
                        ticks: AggrBreacherTicks::Complex1(AggrBreacherTicksComplex1 {
                            dmg_tick_count: early_full_cycle_ticks * reload2.inner_early.repeat_count + opc.tick_count,
                            inactive_tick_count: final_full_cycle_ticks - opc.tick_count,
                            repeat_count: InfCount::Infinite,
                        }),
                    });
                };
            }
            let early_dmg_tick_count = early_full_cycle_ticks.min(opc.tick_count);
            let final_dmg_tick_count = final_full_cycle_ticks.min(opc.tick_count);
            Some(AggrBreacher {
                absolute_max: opc.absolute_max,
                relative_max: opc.relative_max,
                ticks: AggrBreacherTicks::Complex2(AggrBreacherTicksComplex2 {
                    inner_early: AggrBreacherTicksInner {
                        dmg_tick_count: early_dmg_tick_count,
                        inactive_tick_count: early_full_cycle_ticks - early_dmg_tick_count,
                        repeat_count: reload2.inner_early.repeat_count,
                    },
                    inner_final: AggrBreacherTicksInner {
                        dmg_tick_count: final_dmg_tick_count,
                        inactive_tick_count: final_full_cycle_ticks - final_dmg_tick_count,
                        repeat_count: reload2.inner_final.repeat_count,
                    },
                }),
            })
        }
    }
}

pub(in crate::svc::vast) struct BreacherAccum {
    data: RMap<AggrBreacher, Count>,
}
impl BreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(&mut self, opc: OutputDmgBreacher, cycle: Cycle) {
        let aggr = match aggregate_breacher(opc, cycle) {
            Some(aggr) => aggr,
            None => return,
        };
        // Discard all finite effects here, since for now accumulator is used just for dps calcs
        if !aggr.ticks.is_infinite() {
            return;
        }
        let cycle_ticks = aggr.ticks.get_cycle_ticks();
        self.data.insert(aggr, cycle_ticks);
    }
    pub(in crate::svc::vast) fn get_dps(&mut self) -> Option<StatDmgBreacher> {
        if self.data.is_empty() {
            return None;
        };
        // When all the breachers are permanently applied, just pick max for both parameters
        if self
            .data
            .keys()
            .all(|v| matches!(v.ticks, AggrBreacherTicks::Simple(_)))
        {
            return Some(StatDmgBreacher {
                absolute_max: self.data.keys().map(|v| v.absolute_max).max().unwrap() / SERVER_TICK_S,
                relative_max: self.data.keys().map(|v| v.relative_max).max().unwrap() / SERVER_TICK_S,
            });
        }
        // General solution is go tick-to-tick until all modules are restarted, pick max for each
        // tick, and then calculate average
        let total_ticks = self.data.values().copied().reduce(num_integer::lcm).unwrap();
        None
    }
}
