use crate::{api::AbilityMut, misc::EffectMode, ud::UEffectUpdates};

impl<'a> AbilityMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        // Only abilities which exist in source are exposed by API, just unwrap
        let r_abil = self.sol.u_data.src.get_ability(&self.abil_id).unwrap();
        let effect_id = r_abil.effect_id;
        let effect_mode = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::ForceStop,
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_effect_id_mode(self.fighter_key, effect_id, effect_mode, &mut reuse_eupdates);
    }
}
