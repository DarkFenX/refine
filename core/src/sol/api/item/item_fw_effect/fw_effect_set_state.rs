use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::FwEffectMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fw_effect_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_fw_effect = self.uad.items.get_mut(item_key).get_fw_effect_mut().unwrap();
        let old_a_state = uad_fw_effect.get_a_state();
        uad_fw_effect.set_fw_effect_state(state);
        let new_a_state = uad_fw_effect.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_fw_effect_state(self.key, state)
    }
}
