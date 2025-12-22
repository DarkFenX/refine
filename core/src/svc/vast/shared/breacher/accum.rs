use std::collections::hash_map::Entry;

use super::{conv::cycle_to_ticks, ticks::AggrBreacherTicksLooped};
use crate::{
    def::{AttrVal, Count, OF, SERVER_TICK_HZ},
    svc::{cycle::Cycle, output::OutputDmgBreacher, vast::StatDmgBreacher},
    util::RMap,
};

const DAY_TICKS: Count = 24 * 60 * 60 * SERVER_TICK_HZ as Count;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AggrBreacher {
    absolute_max: AttrVal,
    relative_max: AttrVal,
    ticks: AggrBreacherTicksLooped,
}

pub(in crate::svc::vast) struct BreacherAccum {
    data: RMap<AggrBreacher, Count>,
}
impl BreacherAccum {
    pub(in crate::svc::vast) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::vast) fn add(&mut self, opc: OutputDmgBreacher, cycle: Cycle) {
        let ticks = match cycle_to_ticks(cycle, opc.tick_count) {
            Some(ticks) => ticks,
            None => return,
        };
        // Discard all finite effects here, since for now accumulator is used just for dps calcs
        let looped_ticks = match ticks.get_looped_part() {
            Some(looped_ticks) => looped_ticks,
            None => return,
        };
        let aggr = AggrBreacher {
            absolute_max: opc.absolute_max,
            relative_max: opc.relative_max,
            ticks: looped_ticks,
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
                    absolute_max: best_breacher_abs.absolute_max * SERVER_TICK_HZ as f64,
                    relative_max: best_breacher_rel.relative_max * SERVER_TICK_HZ as f64,
                }
                .nullified();
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
            absolute_max: total_abs / total_ticks as f64 * SERVER_TICK_HZ as f64,
            relative_max: total_rel / total_ticks as f64 * SERVER_TICK_HZ as f64,
        }
        .nullified()
    }
}
