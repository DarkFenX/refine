use crate::defs::{EEffectId, SsItemId};

use super::{misc::RunningEffects, svce_calc::SsSvcCalcData};

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss::svc) running_effects: RunningEffects,
    pub(in crate::ss::svc) calc_data: SsSvcCalcData,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self {
            running_effects: RunningEffects::new(),
            calc_data: SsSvcCalcData::new(),
        }
    }
    pub(in crate::ss) fn is_effect_running(&self, item_id: &SsItemId, effect_id: &EEffectId) -> bool {
        self.running_effects.is_running(item_id, effect_id)
    }
}
