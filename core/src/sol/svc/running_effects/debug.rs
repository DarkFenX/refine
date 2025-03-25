use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_item_id},
    uad::Uad,
};

use super::RunningEffects;

impl RunningEffects {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (item_id, effect_ids) in self.data.iter() {
            check_item_id(uad, item_id, true)?;
            for effect_id in effect_ids {
                check_a_effect_id(uad, effect_id)?;
            }
        }
        Ok(())
    }
}
