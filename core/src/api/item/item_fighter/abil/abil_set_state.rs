use crate::{api::AbilityMut, misc::EffectMode, ud::UEffectUpdates};

impl<'a> AbilityMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        // Only abilities which exist in source and only for loaded fighters are exposed by API, so
        // just unwrap everything
        let r_abil = self.sol.u_data.r_data.get_ability_by_aid(&self.abil_aid).unwrap();
        let effect_aid = r_abil.effect_aid;
        let u_fighter = self.sol.u_data.items.get(self.fighter_uid).dc_fighter().unwrap();
        let is_defeff = u_fighter.get_defeff_rid().unwrap() == Some(r_abil.effect_rid);
        let effect_mode = match (state, is_defeff) {
            (true, true) => EffectMode::FullCompliance,
            (true, false) => EffectMode::StateCompliance,
            (false, true) => EffectMode::ForceStop,
            (false, false) => EffectMode::FullCompliance,
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_effect_id_mode(self.fighter_uid, effect_aid, effect_mode, &mut reuse_eupdates);
    }
}
