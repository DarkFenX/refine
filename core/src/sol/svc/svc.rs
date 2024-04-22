use crate::defs::{EEffectId, SolItemId};

use super::{misc::SolRunningEffects, svce_calc::SolSvcCalcData};

pub(in crate::sol) struct SolSvcs {
    pub(in crate::sol::svc) running_effects: SolRunningEffects,
    pub(in crate::sol::svc) calc_data: SolSvcCalcData,
}
impl SolSvcs {
    pub(in crate::sol) fn new() -> Self {
        Self {
            running_effects: SolRunningEffects::new(),
            calc_data: SolSvcCalcData::new(),
        }
    }
    pub(in crate::sol) fn is_effect_running(&self, item_id: &SolItemId, effect_id: &EEffectId) -> bool {
        self.running_effects.is_running(item_id, effect_id)
    }
}
