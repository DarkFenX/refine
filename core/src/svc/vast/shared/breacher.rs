use crate::{
    def::{AttrVal, Count, SERVER_TICK_S},
    svc::{cycle::Cycle, output::OutputDmgBreacher},
    util::{InfCount, ceil_unerr},
};

struct AggrBreacher {
    absolute_max: AttrVal,
    relative_max: AttrVal,
    ticks: AggrBreacherTicks,
}

enum AggrBreacherTicks {
    Simple(InfCount),
    Complex1(AggrBreacherTicksComplex1),
    Complex2(AggrBreacherTicksComplex2),
}

struct AggrBreacherTicksComplex1 {
    dmg_tick_count: Count,
    inactive_tick_count: Count,
    repeat_count: InfCount,
}

struct AggrBreacherTicksInner {
    dmg_tick_count: Count,
    inactive_tick_count: Count,
    repeat_count: Count,
}

// This one is implied to repeat infinitely
struct AggrBreacherTicksComplex2 {
    inner_early: AggrBreacherTicksInner,
    inner_final: AggrBreacherTicksInner,
}

// Process breacher module cycle data + output per cycle into some kind of aggregated value, which
// discards all overlapping instances and aligns everything to ticks, which is needed for further
// processing
fn process_breacher(opc: OutputDmgBreacher, cycle: Cycle) -> Option<AggrBreacher> {
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
