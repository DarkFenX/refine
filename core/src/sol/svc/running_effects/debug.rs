use crate::sol::{
    svc::debug::{check_effect, check_item},
    uad::SolUad,
    SolDebugResult,
};

use super::SolRunningEffects;

impl SolRunningEffects {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (item_id, effect_ids) in self.data.iter() {
            check_item(uad, item_id, true)?;
            for effect_id in effect_ids {
                check_effect(uad, effect_id)?;
            }
        }
        Ok(())
    }
}
