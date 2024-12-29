use crate::{
    defs::{AttrVal, SolItemId, OF},
    util::{sig_round, StMap},
};

use super::{
    rah_data_sim::SolRahDataSim,
    shared::{SIG_DIGITS, TICK_LIMIT},
};

struct SolRahDataIter {
    cycle_time: AttrVal,
    cycle_time_rounded: AttrVal,
    cycling_time: AttrVal,
}
impl SolRahDataIter {
    fn new(cycle_time: AttrVal) -> Self {
        Self {
            cycle_time,
            cycle_time_rounded: sig_round(cycle_time, SIG_DIGITS),
            cycling_time: OF(0.0),
        }
    }
}

pub(super) struct SolRahSimTickData {
    pub(super) time_passed: AttrVal,
    pub(super) cycled: Vec<SolItemId>,
    pub(super) cycling_times: StMap<SolItemId, AttrVal>,
}
impl SolRahSimTickData {
    fn new(time_passed: AttrVal, cycled: Vec<SolItemId>, cycling_times: StMap<SolItemId, AttrVal>) -> Self {
        Self {
            time_passed,
            cycled,
            cycling_times,
        }
    }
}

pub(super) struct SolRahSimTickIter {
    tick: usize,
    rah_iter_data: StMap<SolItemId, SolRahDataIter>,
}
impl SolRahSimTickIter {
    pub(super) fn new<'a>(sim_datas: impl ExactSizeIterator<Item = (&'a SolItemId, &'a SolRahDataSim)>) -> Self {
        let mut iter_datas = StMap::with_capacity(sim_datas.len());
        for (item_id, sim_data) in sim_datas {
            iter_datas.insert(*item_id, SolRahDataIter::new(sim_data.info.cycle_time));
        }
        Self {
            tick: 0,
            rah_iter_data: iter_datas,
        }
    }
}
impl Iterator for SolRahSimTickIter {
    type Item = SolRahSimTickData;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.tick >= TICK_LIMIT {
                return None;
            }
            self.tick += 1;
            // Pick time remaining until some RAH finishes its cycle
            let time_passed = self
                .rah_iter_data
                .values()
                .map(|v| v.cycle_time - v.cycling_time)
                .min()
                .unwrap();
            // Compose list of RAHs which finish their cycle this tick
            let mut cycled = Vec::new();
            for (item_id, item_iter_data) in self.rah_iter_data.iter() {
                // Have time tolerance to cancel float calculation errors. It's needed for multi-RAH
                // configurations, e.g. when normal RAH does 17 cycles, heated one does 20, but
                // >>> sum([0.85] * 20) == 17
                // False
                if sig_round(item_iter_data.cycling_time + time_passed, SIG_DIGITS) >= item_iter_data.cycle_time_rounded
                {
                    cycled.push(*item_id);
                }
            }
            // Update iterator state
            for (item_id, item_iter_data) in self.rah_iter_data.iter_mut() {
                match cycled.contains(item_id) {
                    true => item_iter_data.cycling_time = OF(0.0),
                    false => item_iter_data.cycling_time += time_passed,
                }
            }
            let cycling_times = self.rah_iter_data.iter().map(|(k, v)| (*k, v.cycling_time)).collect();
            return Some(SolRahSimTickData::new(time_passed, cycled, cycling_times));
        }
    }
}
