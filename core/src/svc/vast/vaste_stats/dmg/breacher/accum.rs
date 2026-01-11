use std::collections::hash_map::Entry;

use super::{conv::cseq_to_ticks, ticks::AggrBreacherTicksLooped};
use crate::{
    def::SERVER_TICK_HZ,
    num::{Count, PValue, UnitInterval},
    svc::{
        cycle::{CycleDataTime, CycleSeq},
        output::OutputDmgBreacher,
        vast::StatDmgBreacher,
    },
    util::RMap,
};

const DAY_TICKS: Count = Count::from_u32(24 * 60 * 60 * SERVER_TICK_HZ as u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacher {
    absolute_max: PValue,
    relative_max: UnitInterval,
    ticks: AggrBreacherTicksLooped,
}

pub(in crate::svc::vast::vaste_stats::dmg) struct BreacherAccum {
    data: RMap<AggrBreacher, Count>,
}
impl BreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(&mut self, opc: OutputDmgBreacher, cseq: CycleSeq<CycleDataTime>) {
        let ticks = match cseq_to_ticks(cseq, opc.tick_count) {
            Some(ticks) => ticks,
            None => return,
        };
        // Discard all finite effects here, since for now accumulator is used just for dps calcs
        let ticks_loop = match ticks.get_loop() {
            Some(ticks_loop) => ticks_loop,
            None => return,
        };
        let aggr = AggrBreacher {
            absolute_max: opc.absolute_max,
            relative_max: opc.relative_max,
            ticks: ticks_loop,
        };
        match self.data.entry(aggr) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(aggr.ticks.get_loop_len());
            }
        }
    }
    pub(in crate::svc::vast) fn get_dps(&self) -> Option<StatDmgBreacher> {
        if self.data.is_empty() {
            return None;
        };
        // If breachers with max damage is applying its damage without downtime, no complex calcs
        // needed
        let best_breacher_abs = self.data.keys().max_by_key(|v| v.absolute_max).unwrap();
        if matches!(best_breacher_abs.ticks, AggrBreacherTicksLooped::Is(_)) {
            let best_breacher_rel = self.data.keys().max_by_key(|v| v.relative_max).unwrap();
            if matches!(best_breacher_rel.ticks, AggrBreacherTicksLooped::Is(_)) {
                return StatDmgBreacher {
                    absolute_max: best_breacher_abs.absolute_max * PValue::SERVER_TICK_HZ,
                    relative_max: best_breacher_rel.relative_max.into_pvalue() * PValue::SERVER_TICK_HZ,
                }
                .nullified();
            }
        }
        // General solution is go tick-to-tick until items are looped, pick max for each tick, and
        // then calculate average. Total count of ticks we consider is limited by 1 day to avoid
        // excessively cpu-heavy configurations
        let total_ticks = Count::from_u32(
            self.data
                .values()
                .map(|v| v.into_u32())
                .reduce(num_integer::lcm)
                .unwrap(),
        )
        .min(DAY_TICKS);
        let mut dmg_data = RMap::new();
        for tick in Count::ZERO..total_ticks {
            let mut tick_max_abs = PValue::ZERO;
            let mut tick_max_rel = PValue::ZERO;
            for breacher in self.data.keys() {
                if breacher.ticks.is_applied_on_tick(tick) {
                    tick_max_abs = tick_max_abs.max(breacher.absolute_max);
                    tick_max_rel = tick_max_rel.max(breacher.relative_max.into_pvalue());
                }
            }
            match dmg_data.entry((tick_max_abs, tick_max_rel)) {
                Entry::Occupied(mut entry) => *entry.get_mut() += Count::ONE,
                Entry::Vacant(entry) => {
                    entry.insert(Count::ONE);
                }
            }
        }
        let (total_abs, total_rel) = dmg_data
            .into_iter()
            .map(|((abs, rel), mul)| (abs * mul.into_pvalue(), rel * mul.into_pvalue()))
            .reduce(|(l_abs, l_rel), (r_abs, r_rel)| (l_abs + r_abs, l_rel + r_rel))
            .unwrap();
        StatDmgBreacher {
            absolute_max: total_abs / total_ticks.into_pvalue() * PValue::SERVER_TICK_HZ,
            relative_max: total_rel / total_ticks.into_pvalue() * PValue::SERVER_TICK_HZ,
        }
        .nullified()
    }
}
