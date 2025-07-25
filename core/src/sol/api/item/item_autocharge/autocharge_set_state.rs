use crate::{
    sol::{SolarSystem, api::AutochargeMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_autocharge_state(
        &mut self,
        item_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_autocharge = self.u_data.items.get_mut(item_key).get_autocharge_mut().unwrap();
        let old_a_state = u_autocharge.get_a_state();
        u_autocharge.set_force_disable(!state, reuse_eupdates, &self.u_data.src);
        let new_a_state = u_autocharge.get_a_state();
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

impl<'a> AutochargeMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_autocharge_state(self.key, state, &mut reuse_eupdates)
    }
}
