use lender::{Lender, Lending};

use super::{
    rah_data_sim::RahDataSim,
    shared::{TICK_LIMIT, TickCount, rah_round},
};
use crate::{
    def::{AttrVal, OF},
    ud::UItemKey,
    util::RMap,
};

struct RahDataIter {
    cycle_time: AttrVal,
    cycle_time_rounded: AttrVal,
    cycling_time: AttrVal,
}
impl RahDataIter {
    fn new(cycle_time: AttrVal) -> Self {
        Self {
            cycle_time,
            cycle_time_rounded: rah_round(cycle_time),
            cycling_time: OF(0.0),
        }
    }
}

pub(super) struct RahSimTickData<'a> {
    pub(super) time_passed: AttrVal,
    pub(super) cycled: &'a Vec<UItemKey>,
    pub(super) cycling_times: &'a RMap<UItemKey, AttrVal>,
}

pub(super) struct RahSimTickIter {
    tick: TickCount,
    rah_iter_data: RMap<UItemKey, RahDataIter>,
    // Fields exposed in iter items
    cycled: Vec<UItemKey>,
    cycling_times: RMap<UItemKey, AttrVal>,
}
impl RahSimTickIter {
    pub(super) fn new<'a>(sim_datas: impl ExactSizeIterator<Item = (&'a UItemKey, &'a RahDataSim)>) -> Self {
        let mut iter_datas = RMap::with_capacity(sim_datas.len());
        for (&item_key, sim_data) in sim_datas {
            iter_datas.insert(item_key, RahDataIter::new(sim_data.info.cycle_time));
        }
        Self {
            tick: 0,
            rah_iter_data: iter_datas,
            cycled: Vec::new(),
            cycling_times: RMap::new(),
        }
    }
}
impl<'lend> Lending<'lend> for RahSimTickIter {
    type Lend = RahSimTickData<'lend>;
}
impl Lender for RahSimTickIter {
    fn next(&mut self) -> Option<RahSimTickData<'_>> {
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
        self.cycled.clear();
        for (item_key, item_iter_data) in self.rah_iter_data.iter() {
            // Have time tolerance to cancel float calculation errors. It's needed for multi-RAH
            // configurations which the engine allows, e.g. when normal RAH does 17 cycles,
            // heated one does 20, but sum of 20x 0.85 f64's is less than 17.
            if rah_round(item_iter_data.cycling_time + time_passed) >= item_iter_data.cycle_time_rounded {
                self.cycled.push(*item_key);
            }
        }
        // Update iterator state
        for (item_key, item_iter_data) in self.rah_iter_data.iter_mut() {
            match self.cycled.contains(item_key) {
                true => item_iter_data.cycling_time = OF(0.0),
                false => item_iter_data.cycling_time += time_passed,
            }
        }
        self.cycling_times.clear();
        self.cycling_times
            .extend(self.rah_iter_data.iter().map(|(k, v)| (*k, v.cycling_time)));
        Some(RahSimTickData {
            time_passed,
            cycled: &self.cycled,
            cycling_times: &self.cycling_times,
        })
    }
}
