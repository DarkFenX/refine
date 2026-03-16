use super::ticks::{AbtCycleComplex, AbtCycleSimple, AbtInfinite, AggrBreacherTicks};
use crate::{
    misc::Breacher,
    num::{Count, PValue},
    svc::{
        cycle::{CycleDataDur, CycleSeq},
        output::Output,
    },
};

// Process breacher module cycle sequence + output per cycle into some kind of aggregated value,
// which discards all overlapping instances and aligns everything to ticks, which is needed for
// further processing
pub(super) fn cseq_to_ticks(cseq: CycleSeq<CycleDataDur>, output: Output<Breacher>) -> Option<AggrBreacherTicks> {
    // Breacher aggregator only supports simple output, and only simple output is used
    let output = match output {
        Output::Simple(output) => output,
        Output::Complex(_) => return None,
    };
    let output_ticks = output.instance.tick_count;
    if output_ticks < Count::ONE {
        return None;
    }
    // Breacher aggregator supports only 2 cycle sequence types: looped limited-single, and infinite
    // (which is degenerate case of looped limited-single cycle sequence)
    match cseq {
        CycleSeq::LoopLimSin(inner) => {
            let delay_ticks = duration_to_ticks(output.delay);
            let cycle_p1_ticks = duration_to_ticks(inner.p1_data.duration);
            let cycle_p2_ticks = duration_to_ticks(inner.p2_data.duration);
            match (output_ticks >= cycle_p1_ticks, output_ticks >= cycle_p2_ticks) {
                (true, true) => Some(AggrBreacherTicks::Infinite(AbtInfinite {
                    initial_delay: delay_ticks,
                })),
                (true, false) => Some(AggrBreacherTicks::CycleSimple(AbtCycleSimple {
                    initial_delay: delay_ticks,
                    cycle_dmg: cycle_p1_ticks * inner.p1_repeat_count + output_ticks,
                    cycle_total: cycle_p1_ticks * inner.p1_repeat_count + cycle_p2_ticks,
                })),
                _ => Some(AggrBreacherTicks::CycleComplex(AbtCycleComplex {
                    initial_delay: delay_ticks,
                    cycle_p1_dmg: output_ticks.min(cycle_p1_ticks),
                    cycle_p1_total: cycle_p1_ticks,
                    cycle_p1_repeats: inner.p1_repeat_count,
                    cycle_p2_dmg: output_ticks.min(cycle_p2_ticks),
                    cycle_p2_total: cycle_p2_ticks,
                })),
            }
        }
        CycleSeq::Inf(inner) => {
            let delay_ticks = duration_to_ticks(output.delay);
            let cycle_ticks = duration_to_ticks(inner.data.duration);
            match output_ticks >= cycle_ticks {
                true => Some(AggrBreacherTicks::Infinite(AbtInfinite {
                    initial_delay: delay_ticks,
                })),
                false => Some(AggrBreacherTicks::CycleSimple(AbtCycleSimple {
                    initial_delay: delay_ticks,
                    cycle_dmg: output_ticks,
                    cycle_total: cycle_ticks,
                })),
            }
        }
        _ => None,
    }
}

fn duration_to_ticks(duration: PValue) -> Count {
    Count::from_pvalue_ceiled(duration * PValue::SERVER_TICK_HZ)
}
