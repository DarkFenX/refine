use std::collections::hash_map::Entry;

use crate::{
    ac,
    def::{AttrVal, Count, OF, SERVER_TICK_HZ, SERVER_TICK_S},
    svc::{SvcCtx, calc::Calc, cycle::Cycle, output::OutputDmgBreacher, vast::StatDmgBreacher},
    ud::UItemKey,
    util::{InfCount, RMap, ceil_unerr},
};

const DAY_TICKS: Count = 24 * 60 * 60 * SERVER_TICK_HZ as Count;

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
    fn get_ticks_per_cycle(&self) -> Count {
        match &self {
            Self::Simple(_) => 1,
            Self::Complex1(complex1) => complex1.get_ticks_per_cycle(),
            Self::Complex2(complex2) => complex2.get_ticks_per_cycle(),
        }
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        match self {
            Self::Simple(count) => match count {
                InfCount::Count(count) => tick < *count,
                InfCount::Infinite => true,
            },
            Self::Complex1(complex1) => complex1.is_applied_on_tick(tick),
            Self::Complex2(complex2) => complex2.is_applied_on_tick(tick),
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
    fn get_ticks_per_cycle(&self) -> Count {
        self.dmg_tick_count + self.inactive_tick_count
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let ticks_per_cycle = self.dmg_tick_count + self.inactive_tick_count;
        if let InfCount::Count(repeat_count) = self.repeat_count
            && tick / ticks_per_cycle >= repeat_count
        {
            return false;
        };
        let in_cycle_tick = tick % ticks_per_cycle;
        in_cycle_tick < self.dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacherTicksInner {
    dmg_tick_count: Count,
    inactive_tick_count: Count,
    repeat_count: Count,
}
impl AggrBreacherTicksInner {
    fn get_ticks_per_cycle(&self) -> Count {
        (self.dmg_tick_count + self.inactive_tick_count) * self.repeat_count
    }
    fn is_applied_on_tick(&self, inner_tick: Count) -> bool {
        // Caller should guarantee that requested tick is within total cycle bound
        let ticks_per_inner_cycle = self.dmg_tick_count + self.inactive_tick_count;
        let in_cycle_tick = inner_tick % ticks_per_inner_cycle;
        in_cycle_tick < self.dmg_tick_count
    }
}

// This one is implied to repeat infinitely
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacherTicksComplex2 {
    inner_early: AggrBreacherTicksInner,
    inner_final: AggrBreacherTicksInner,
}
impl AggrBreacherTicksComplex2 {
    fn get_ticks_per_cycle(&self) -> Count {
        self.inner_early.get_ticks_per_cycle() + self.inner_final.get_ticks_per_cycle()
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let early_ticks = self.inner_early.get_ticks_per_cycle();
        let final_ticks = self.inner_early.get_ticks_per_cycle();
        let ticks_per_full_cycle = early_ticks + final_ticks;
        let in_full_cycle_tick = tick % ticks_per_full_cycle;
        if in_full_cycle_tick < early_ticks {
            return self.inner_early.is_applied_on_tick(in_full_cycle_tick);
        }
        self.inner_final.is_applied_on_tick(in_full_cycle_tick - early_ticks)
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
            let full_cycle_ticks =
                ceil_unerr((simple.active_time + simple.inactive_time) / SERVER_TICK_S).into_inner() as Count;
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
            let full_cycle_ticks = ceil_unerr((reload1.inner.active_time + reload1.inner.inactive_time) / SERVER_TICK_S)
                .into_inner() as Count;
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
                    .into_inner() as Count;
            let final_full_cycle_ticks =
                ceil_unerr((reload2.inner_final.active_time + reload2.inner_final.inactive_time) / SERVER_TICK_S)
                    .into_inner() as Count;
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
        match self.data.entry(aggr) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(aggr.ticks.get_ticks_per_cycle());
            }
        }
    }
    pub(in crate::svc::vast) fn get_dps(&self) -> Option<StatDmgBreacher> {
        if self.data.is_empty() {
            return None;
        };
        // If breachers with max damage are infinitely cycling, no complex calcs needed
        let best_breacher_abs = self.data.keys().max_by_key(|v| v.absolute_max).unwrap();
        if matches!(best_breacher_abs.ticks, AggrBreacherTicks::Simple(_)) {
            let best_breacher_rel = self.data.keys().max_by_key(|v| v.relative_max).unwrap();
            if matches!(best_breacher_rel.ticks, AggrBreacherTicks::Simple(_)) {
                return StatDmgBreacher {
                    absolute_max: best_breacher_abs.absolute_max / SERVER_TICK_S,
                    relative_max: best_breacher_rel.relative_max / SERVER_TICK_S,
                }
                .nullify();
            }
        }
        // General solution is go tick-to-tick until items are looped, pick max for each tick, and
        // then calculate average. Total count of ticks we consider is limited by 1 day to avoid
        // excessively cpu-heavy configurations
        let total_ticks = self
            .data
            .values()
            .copied()
            .reduce(num_integer::lcm)
            .unwrap()
            .min(DAY_TICKS);
        let mut dmg_data = RMap::new();
        for tick in 0..total_ticks {
            let mut tick_max_abs = OF(0.0);
            let mut tick_max_rel = OF(0.0);
            for breacher in self.data.keys() {
                if breacher.ticks.is_applied_on_tick(tick) {
                    tick_max_abs = tick_max_abs.max(breacher.absolute_max);
                    tick_max_rel = tick_max_rel.max(breacher.relative_max);
                }
            }
            match dmg_data.entry((tick_max_abs, tick_max_rel)) {
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
        let (total_abs, total_rel) = dmg_data
            .into_iter()
            .map(|((abs, rel), mul)| (abs * mul as f64, rel * mul as f64))
            .reduce(|(l_abs, l_rel), (r_abs, r_rel)| (l_abs + r_abs, l_rel + r_rel))
            .unwrap();
        StatDmgBreacher {
            absolute_max: total_abs / total_ticks as f64 / SERVER_TICK_S,
            relative_max: total_rel / total_ticks as f64 / SERVER_TICK_S,
        }
        .nullify()
    }
}

pub(in crate::svc::vast) fn apply_breacher(
    ctx: SvcCtx,
    calc: &mut Calc,
    breacher_raw: StatDmgBreacher,
    projectee_key: UItemKey,
) -> AttrVal {
    let hp_shield = calc
        .get_item_attr_val_extra(ctx, projectee_key, &ac::attrs::SHIELD_CAPACITY)
        .unwrap_or(OF(0.0));
    let hp_armor = calc
        .get_item_attr_val_extra(ctx, projectee_key, &ac::attrs::ARMOR_HP)
        .unwrap_or(OF(0.0));
    let hp_hull = calc
        .get_item_attr_val_extra(ctx, projectee_key, &ac::attrs::HP)
        .unwrap_or(OF(0.0));
    breacher_raw
        .absolute_max
        .min(breacher_raw.relative_max * (hp_shield + hp_armor + hp_hull))
}
