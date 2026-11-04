use crate::{Ability, EffectMode};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Ability<'s> {
    pub fn get_state(&self) -> bool {
        let abil_id = &self.abil_aid;
        // Only abilities which exist in source are exposed by API, just unwrap
        let r_abil = self.core.u_data.r_data.get_ability_by_aid(abil_id).unwrap();
        let u_fighter = self.core.u_data.items.get(self.fighter_uid).dc_fighter().unwrap();
        match u_fighter.get_effect_mode_by_rid(&r_abil.effect_rid) {
            // Default disabled/active effects are ran if fighter becomes disabled+/active+, others
            // are not
            EffectMode::FullCompliance => Some(r_abil.effect_rid) == u_fighter.get_r_item_base().unwrap().defeff_rid,
            EffectMode::StateCompliance => true,
            EffectMode::ForceRun => true,
            EffectMode::ForceStop => false,
        }
    }
}
