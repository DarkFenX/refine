use lender::{Lender, Lending, check_covariance};

use super::{
    item_data::ItemData,
    shared::{ItemDataVec, SIG_ROUND_DIGITS, TICK_LIMIT, TickCount},
};
use crate::num::PValue;

struct IterItemInfo {
    cycle_duration: PValue,
    cycle_duration_rounded: PValue,
    cycle_time: PValue,
}
impl IterItemInfo {
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
    pub(super) cycled: &'a ItemDataVec<usize>,
    pub(super) cycle_times: &'a ItemDataVec<PValue>,
}

pub(super) struct RahSimTickIter {
    tick: TickCount,
    item_infos: ItemDataVec<IterItemInfo>,
    // Fields exposed in iter items
    cycled: ItemDataVec<usize>,
    cycle_times: ItemDataVec<PValue>,
}
impl RahSimTickIter {
    pub(super) fn new<'a>(item_datas: impl ExactSizeIterator<Item = &'a ItemData>) -> Self {
        let mut item_infos = ItemDataVec::with_capacity(item_datas.len());
        let mut cycle_times = ItemDataVec::with_capacity(item_datas.len());
        for item_data in item_datas {
            item_infos.push(IterItemInfo::new(item_data.info.cycle_duration));
            cycle_times.push(PValue::ZERO);
        }
        Self {
            tick: 0,
            item_infos,
            cycled: ItemDataVec::new(),
            cycle_times,
        }
    }
}
impl<'lend> Lending<'lend> for RahSimTickIter {
    type Lend = RahSimTickData<'lend>;
}
impl Lender for RahSimTickIter {
    check_covariance!();

    fn next(&mut self) -> Option<RahSimTickData<'_>> {
        if self.tick >= TICK_LIMIT {
            return None;
        }
        self.tick += 1;
        // Clear state exposed to iter caller
        self.cycled.clear();
        // Pick time remaining until some RAH finishes its cycle
        let time_passed = PValue::from_value_clamped(
            self.item_infos
                .iter()
                .map(|v| v.cycle_duration - v.cycle_time)
                .min()
                .unwrap(),
        );
        // Check which RAHs finish their cycle this tick
        for (item_idx, item_info) in self.item_infos.iter_mut().enumerate() {
            // Have time tolerance to cancel float calculation errors. It's needed for multi-RAH
            // configurations which the engine allows, e.g. when normal RAH does 17 cycles, heated
            // one does 20, but sum of 20x 0.85 f64's is less than 17.
            match (item_info.cycle_time + time_passed).sig_rounded(SIG_ROUND_DIGITS) >= item_info.cycle_duration_rounded
            {
                true => {
                    item_info.cycle_time = PValue::ZERO;
                    self.cycled.push(item_idx);
                }
                false => item_info.cycle_time += time_passed,
            }
            *self.cycle_times.get_mut(item_idx) = item_info.cycle_time;
        }
        Some(RahSimTickData {
            time_passed,
            cycled: &self.cycled,
            cycle_times: &self.cycle_times,
        })
    }
}
