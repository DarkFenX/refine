use ordered_float::OrderedFloat as OF;

use crate::{
    sol::{AttrVal, ItemId},
    util::StMap,
};

use super::{
    rah_data_sim::RahDataSim,
    shared::{TICK_LIMIT, TickCount, rah_round},
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

pub(super) struct RahSimTickData {
    pub(super) time_passed: AttrVal,
    pub(super) cycled: Vec<ItemId>,
    pub(super) cycling_times: StMap<ItemId, AttrVal>,
}

pub(super) struct RahSimTickIter {
    tick: TickCount,
    rah_iter_data: StMap<ItemId, RahDataIter>,
}
impl RahSimTickIter {
    pub(super) fn new<'a>(sim_datas: impl ExactSizeIterator<Item = (&'a ItemId, &'a RahDataSim)>) -> Self {
        let mut iter_datas = StMap::with_capacity(sim_datas.len());
        for (item_id, sim_data) in sim_datas {
            iter_datas.insert(*item_id, RahDataIter::new(sim_data.info.cycle_time));
        }
        Self {
            tick: 0,
            rah_iter_data: iter_datas,
        }
    }
}
impl Iterator for RahSimTickIter {
    type Item = RahSimTickData;

    fn next(&mut self) -> Option<Self::Item> {
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
            // configurations which the engine allows, e.g. when normal RAH does 17 cycles,
            // heated one does 20, but sum of 20x 0.85 f64's is less than 17.
            if rah_round(item_iter_data.cycling_time + time_passed) >= item_iter_data.cycle_time_rounded {
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
        Some(RahSimTickData {
            time_passed,
            cycled,
            cycling_times,
        })
    }
}
