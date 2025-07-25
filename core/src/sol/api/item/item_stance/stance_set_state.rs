use crate::{
    sol::{SolarSystem, api::StanceMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_stance_state(
        &mut self,
        item_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_stance = self.u_data.items.get_mut(item_key).get_stance_mut().unwrap();
        let old_a_state = u_stance.get_a_state();
        u_stance.set_stance_state(state, reuse_eupdates, &self.u_data.src);
        let new_a_state = u_stance.get_a_state();
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

impl<'a> StanceMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_stance_state(self.key, state, &mut reuse_eupdates)
    }
}
