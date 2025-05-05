use crate::sol::{ItemKey, SolarSystem, api::ProjEffectMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_proj_effect_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        let old_a_state = uad_proj_effect.get_a_state();
        uad_proj_effect.set_proj_effect_state(state);
        let new_a_state = uad_proj_effect.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_proj_effect_state(self.key, state)
    }
}
