use crate::{
    defs::{AttrVal, SolItemId},
    util::{sig_round, StMap},
};

use super::{
    info::SolRahSimRahData,
    shared::{SIG_DIGITS, TICK_LIMIT},
};

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

pub(super) struct SolRahSimTickIterRahData {
    info: SolRahSimRahData,
    cycling_time: AttrVal,
}
impl SolRahSimTickIterRahData {
    fn new(info: SolRahSimRahData) -> Self {
        Self {
            info,
            cycling_time: 0.0,
        }
    }
}

pub(super) struct SolRahSimTickIter {
    tick: usize,
    rah_data: StMap<SolItemId, SolRahSimTickIterRahData>,
}
impl SolRahSimTickIter {
    pub(super) fn new(infos: &StMap<SolItemId, SolRahSimRahData>) -> Self {
        let mut rah_data = StMap::with_capacity(infos.len());
        for (item_id, info) in infos.iter() {
            rah_data.insert(*item_id, SolRahSimTickIterRahData::new(*info));
        }
        Self { tick: 0, rah_data }
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
                .rah_data
                .values()
                .map(|v| v.info.cycle_time - v.cycling_time)
                .min_by(|a, b| a.total_cmp(b))
                .unwrap();
            // Compose list of RAHs which finish their cycle this tick
            let mut cycled = Vec::new();
            for (item_id, item_data) in self.rah_data.iter() {
                // Have time tolerance to cancel float calculation errors. It's needed for multi-RAH
                // configurations, e.g. when normal RAH does 17 cycles, heated one does 20, but
                // >>> sum([0.85] * 20) == 17
                // False
                if sig_round(item_data.cycling_time + time_passed, SIG_DIGITS) >= item_data.info.cycle_time_rounded {
                    cycled.push(*item_id);
                }
            }
            // Update iterator state
            for (item_id, item_data) in self.rah_data.iter_mut() {
                match cycled.contains(item_id) {
                    true => item_data.cycling_time = 0.0,
                    false => item_data.cycling_time += time_passed,
                }
            }
            let cycling_times = self.rah_data.iter().map(|(k, v)| (*k, v.cycling_time)).collect();
            return Some(SolRahSimTickData::new(time_passed, cycled, cycling_times));
        }
    }
}
