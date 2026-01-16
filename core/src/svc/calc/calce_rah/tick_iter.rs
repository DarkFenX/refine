use lender::{Lender, Lending};

use super::{
    rah_data_sim::RahDataSim,
    shared::{SIG_ROUND_DIGITS, TICK_LIMIT, TickCount},
};
use crate::{num::PValue, ud::UItemId, util::RMap};

struct RahDataIter {
    cycle_duration: PValue,
    cycle_duration_rounded: PValue,
    cycle_time: PValue,
}
impl RahDataIter {
    fn new(cycle_duration: PValue) -> Self {
        Self {
            cycle_duration,
            cycle_duration_rounded: cycle_duration.sig_rounded(SIG_ROUND_DIGITS),
            cycle_time: PValue::ZERO,
        }
    }
}

pub(super) struct RahSimTickData<'a> {
    pub(super) time_passed: PValue,
    pub(super) cycled: &'a Vec<UItemId>,
    pub(super) cycle_times: &'a RMap<UItemId, PValue>,
}

pub(super) struct RahSimTickIter {
    tick: TickCount,
    rah_iter_data: RMap<UItemId, RahDataIter>,
    // Fields exposed in iter items
    cycled: Vec<UItemId>,
    cycle_times: RMap<UItemId, PValue>,
}
impl RahSimTickIter {
    pub(super) fn new<'a>(sim_datas: impl ExactSizeIterator<Item = (&'a UItemId, &'a RahDataSim)>) -> Self {
        let mut iter_datas = RMap::with_capacity(sim_datas.len());
        for (&item_uid, sim_data) in sim_datas {
            iter_datas.insert(item_uid, RahDataIter::new(sim_data.info.cycle_duration));
        }
        Self {
            tick: 0,
            rah_iter_data: iter_datas,
            cycled: Vec::new(),
            cycle_times: RMap::new(),
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
        // Clear state exposed to iter caller
        self.cycled.clear();
        self.cycle_times.clear();
        // Pick time remaining until some RAH finishes its cycle
        let time_passed = PValue::from_value_clamped(
            self.rah_iter_data
                .values()
                .map(|v| v.cycle_duration - v.cycle_time)
                .min()
                .unwrap(),
        );
        // Compose list of RAHs which finish their cycle this tick
        for (item_uid, item_iter_data) in self.rah_iter_data.iter() {
            // Have time tolerance to cancel float calculation errors. It's needed for multi-RAH
            // configurations which the engine allows, e.g. when normal RAH does 17 cycles,
            // heated one does 20, but sum of 20x 0.85 f64's is less than 17.
            if (item_iter_data.cycle_time + time_passed).sig_rounded(SIG_ROUND_DIGITS)
                >= item_iter_data.cycle_duration_rounded
            {
                self.cycled.push(*item_uid);
            }
        }
        // Update iterator state
        for (item_uid, item_iter_data) in self.rah_iter_data.iter_mut() {
            match self.cycled.contains(item_uid) {
                true => item_iter_data.cycle_time = PValue::ZERO,
                false => item_iter_data.cycle_time += time_passed,
            }
        }
        self.cycle_times
            .extend(self.rah_iter_data.iter().map(|(k, v)| (*k, v.cycle_time)));
        Some(RahSimTickData {
            time_passed,
            cycled: &self.cycled,
            cycle_times: &self.cycle_times,
        })
    }
}
