use std::collections::HashSet;

use calc::CalcData;
pub use calc::SsAttrVal;
use effect::RunningEffects;

use crate::defs::{EffectId, SsItemId};

mod calc;
mod effect;
mod routing;

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss::svc) running_effects: RunningEffects,
    pub(in crate::ss::svc) calc_data: CalcData,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self {
            running_effects: RunningEffects::new(),
            calc_data: CalcData::new(),
        }
    }
    pub(in crate::ss) fn get_running_effects(&self, item_id: &SsItemId) -> Option<&HashSet<EffectId>> {
        self.running_effects.get_running_effects(item_id)
    }
}
