use crate::{
    sol::{SolarSystem, api::SwEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_sw_effect_state(
        &mut self,
        item_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_sw_effect = self.u_data.items.get_mut(item_key).get_sw_effect_mut().unwrap();
        let old_a_state = u_sw_effect.get_state();
        u_sw_effect.set_sw_effect_state(state);
        let new_a_state = u_sw_effect.get_state();
        u_sw_effect.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> SwEffectMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_sw_effect_state(self.key, state, &mut reuse_eupdates)
    }
}
