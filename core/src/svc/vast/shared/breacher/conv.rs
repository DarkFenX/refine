use super::ticks::{AbtIc, AbtIs, AbtLc, AbtLcIc, AbtLcLcIc, AbtLoopLcLc, AbtLs, AggrBreacherTicks};
use crate::{
    def::{AttrVal, Count, SERVER_TICK_HZ},
    svc::cycle::Cycle,
    util::ceil_unerr,
};

// Process breacher module cycle data + output per cycle into some kind of aggregated value, which
// discards all overlapping instances and aligns everything to ticks, which is needed for further
// processing
pub(super) fn cycle_to_ticks(cycle: Cycle, output_ticks: Count) -> Option<AggrBreacherTicks> {
    if output_ticks < 1 {
        return None;
    }
    match cycle {
        Cycle::Limited(limited) => {
            if limited.repeat_count == 0 {
                return None;
            }
            let cycle_ticks = time_to_ticks(limited.active_time + limited.inactive_time);
            match output_ticks >= cycle_ticks {
                true => {
                    let last_cycle_start_ts =
                        (limited.active_time + limited.inactive_time) * (limited.repeat_count - 1) as f64;
                    let last_cycle_start_tick = time_to_ticks(last_cycle_start_ts);
                    Some(AggrBreacherTicks::Ls(AbtLs {
                        count: last_cycle_start_tick + output_ticks,
                    }))
                }
                false => Some(AggrBreacherTicks::Lc(AbtLc {
                    dmg_tick_count: output_ticks,
                    inactive_tick_count: cycle_ticks - output_ticks,
                    repeat_count: limited.repeat_count,
                })),
            }
        }
        Cycle::Infinite1(infinite1) => {
            let cycle_ticks = time_to_ticks(infinite1.active_time + infinite1.inactive_time);
            match output_ticks >= cycle_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => Some(AggrBreacherTicks::Ic(AbtIc {
                    dmg_tick_count: output_ticks,
                    inactive_tick_count: cycle_ticks - output_ticks,
                })),
            }
        }
        Cycle::Infinite2(infinite2) => {
            let p1_ticks = time_to_ticks(infinite2.p1_active_time + infinite2.p1_inactive_time);
            let p2_ticks = time_to_ticks(infinite2.p2_active_time + infinite2.p2_inactive_time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => {
                    let p1_dmg_ticks = output_ticks.min(p1_ticks);
                    let p2_dmg_ticks = output_ticks.min(p2_ticks);
                    Some(AggrBreacherTicks::LcIc(AbtLcIc {
                        p1_dmg_tick_count: p1_dmg_ticks,
                        p1_inactive_tick_count: p1_ticks - p1_dmg_ticks,
                        p1_repeat_count: infinite2.p1_repeat_count,
                        p2_dmg_tick_count: p2_dmg_ticks,
                        p2_inactive_tick_count: p2_ticks - p2_dmg_ticks,
                    }))
                }
            }
        }
        Cycle::Infinite3(infinite3) => {
            let p1_ticks = time_to_ticks(infinite3.p1_active_time + infinite3.p1_inactive_time);
            let p2_ticks = time_to_ticks(infinite3.p2_active_time + infinite3.p2_inactive_time);
            let p3_ticks = time_to_ticks(infinite3.p3_active_time + infinite3.p3_inactive_time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks && output_ticks > p3_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => {
                    let p1_dmg_ticks = output_ticks.min(p1_ticks);
                    let p2_dmg_ticks = output_ticks.min(p2_ticks);
                    let p3_dmg_ticks = output_ticks.min(p3_ticks);
                    Some(AggrBreacherTicks::LcLcIc(AbtLcLcIc {
                        p1_dmg_tick_count: p1_dmg_ticks,
                        p1_inactive_tick_count: p1_ticks - p1_dmg_ticks,
                        p1_repeat_count: infinite3.p1_repeat_count,
                        p2_dmg_tick_count: p2_dmg_ticks,
                        p2_inactive_tick_count: p2_ticks - p2_dmg_ticks,
                        p2_repeat_count: 1,
                        p3_dmg_tick_count: p3_dmg_ticks,
                        p3_inactive_tick_count: p3_ticks - p3_dmg_ticks,
                    }))
                }
            }
        }
        Cycle::Looped2(looped2) => {
            let p1_ticks = time_to_ticks(looped2.p1_active_time + looped2.p1_inactive_time);
            let p2_ticks = time_to_ticks(looped2.p2_active_time + looped2.p2_inactive_time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => {
                    let p1_dmg_ticks = output_ticks.min(p1_ticks);
                    let p2_dmg_ticks = output_ticks.min(p2_ticks);
                    Some(AggrBreacherTicks::LoopLcLc(AbtLoopLcLc {
                        p1_dmg_tick_count: p1_dmg_ticks,
                        p1_inactive_tick_count: p1_ticks - p1_dmg_ticks,
                        p1_repeat_count: looped2.p1_repeat_count,
                        p2_dmg_tick_count: p2_dmg_ticks,
                        p2_inactive_tick_count: p2_ticks - p2_dmg_ticks,
                        p2_repeat_count: 1,
                    }))
                }
            }
        }
    }
}

fn time_to_ticks(time: AttrVal) -> Count {
    ceil_unerr(time * SERVER_TICK_HZ as f64).into_inner() as Count
}
