use crate::{
    api::FwEffectMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fw_effect_state(
        &mut self,
        fw_effect_key: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_fw_effect = self.u_data.items.get_mut(fw_effect_key).dc_fw_effect_mut().unwrap();
        let old_a_state = u_fw_effect.get_state();
        u_fw_effect.set_fw_effect_state(state);
        let new_a_state = u_fw_effect.get_state();
        u_fw_effect.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            fw_effect_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fw_effect_state(self.key, state, &mut reuse_eupdates)
    }
}
