use crate::{misc::EffectMode, sol::api::AbilityMut};

impl<'a> AbilityMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        // Only abilities which exist in source are exposed by API, just unwrap
        let r_abil = self.sol.u_data.src.get_ability(&self.abil_id).unwrap();
        let effect_id = r_abil.get_effect_id();
        let effect_mode = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::ForceStop,
        };
        self.sol
            .internal_set_effect_id_mode(self.item_key, effect_id, effect_mode);
    }
}
