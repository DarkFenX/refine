use crate::{
    sol::{
        svc::debug::{check_effect, check_item},
        SolView,
    },
    util::DebugResult,
};

use super::SolRunningEffects;

impl SolRunningEffects {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (item_id, effect_ids) in self.data.iter() {
            check_item(sol_view, item_id)?;
            for effect_id in effect_ids {
                check_effect(sol_view, effect_id)?;
            }
        }
        Ok(())
    }
}
