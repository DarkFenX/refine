use super::ticks::{AbtIc, AbtIs, AbtLc, AbtLcIc, AbtLcLcIc, AbtLoopLcLc, AbtLs, AggrBreacherTicks};
use crate::{
    def::{AttrVal, DefCount, SERVER_TICK_HZ},
    svc::cycle::{CycleDataTime, CycleSeq},
    util::ceil_unerr,
};

// Process breacher module cycle sequence + output per cycle into some kind of aggregated value,
// which discards all overlapping instances and aligns everything to ticks, which is needed for
// further processing
pub(super) fn cseq_to_ticks(cseq: CycleSeq<CycleDataTime>, output_ticks: DefCount) -> Option<AggrBreacherTicks> {
    if output_ticks < 1 {
        return None;
    }
    match cseq {
        CycleSeq::Lim(inner) => {
            if inner.repeat_count == 0 {
                return None;
            }
            let cycle_ticks = time_to_ticks(inner.data.time);
            match output_ticks >= cycle_ticks {
                true => {
                    let last_cycle_start_ts = inner.data.time * (inner.repeat_count - 1) as f64;
                    let last_cycle_start_tick = time_to_ticks(last_cycle_start_ts);
                    Some(AggrBreacherTicks::Ls(AbtLs {
                        count: last_cycle_start_tick + output_ticks,
                    }))
                }
                false => Some(AggrBreacherTicks::Lc(AbtLc {
                    dmg_tick_count: output_ticks,
                    tick_count: cycle_ticks,
                    repeat_count: inner.repeat_count,
                })),
            }
        }
        CycleSeq::Inf(inner) => {
            let cycle_ticks = time_to_ticks(inner.data.time);
            match output_ticks >= cycle_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => Some(AggrBreacherTicks::Ic(AbtIc {
                    dmg_tick_count: output_ticks,
                    tick_count: cycle_ticks,
                })),
            }
        }
        CycleSeq::LimInf(inner) => {
            let p1_ticks = time_to_ticks(inner.p1_data.time);
            let p2_ticks = time_to_ticks(inner.p2_data.time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => Some(AggrBreacherTicks::LcIc(AbtLcIc {
                    p1_dmg_tick_count: output_ticks.min(p1_ticks),
                    p1_tick_count: p1_ticks,
                    p1_repeat_count: inner.p1_repeat_count,
                    p2_dmg_tick_count: output_ticks.min(p2_ticks),
                    p2_tick_count: p2_ticks,
                })),
            }
        }
        CycleSeq::LimSinInf(inner) => {
            let p1_ticks = time_to_ticks(inner.p1_data.time);
            let p2_ticks = time_to_ticks(inner.p2_data.time);
            let p3_ticks = time_to_ticks(inner.p3_data.time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks && output_ticks > p3_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => Some(AggrBreacherTicks::LcLcIc(AbtLcLcIc {
                    p1_dmg_tick_count: output_ticks.min(p1_ticks),
                    p1_tick_count: p1_ticks,
                    p1_repeat_count: inner.p1_repeat_count,
                    p2_dmg_tick_count: output_ticks.min(p2_ticks),
                    p2_tick_count: p2_ticks,
                    p2_repeat_count: 1,
                    p3_dmg_tick_count: output_ticks.min(p3_ticks),
                    p3_tick_count: p3_ticks,
                })),
            }
        }
        CycleSeq::LoopLimSin(inner) => {
            let p1_ticks = time_to_ticks(inner.p1_data.time);
            let p2_ticks = time_to_ticks(inner.p2_data.time);
            match output_ticks >= p1_ticks && output_ticks >= p2_ticks {
                true => Some(AggrBreacherTicks::Is(AbtIs {})),
                false => Some(AggrBreacherTicks::LoopLcLc(AbtLoopLcLc {
                    p1_dmg_tick_count: output_ticks.min(p1_ticks),
                    p1_tick_count: p1_ticks,
                    p1_repeat_count: inner.p1_repeat_count,
                    p2_dmg_tick_count: output_ticks.min(p2_ticks),
                    p2_tick_count: p2_ticks,
                    p2_repeat_count: 1,
                })),
            }
        }
    }
}

fn time_to_ticks(time: AttrVal) -> DefCount {
    ceil_unerr(time * SERVER_TICK_HZ as f64).into_inner() as DefCount
}
