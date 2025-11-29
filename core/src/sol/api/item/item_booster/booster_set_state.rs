use crate::{
    sol::{SolarSystem, api::BoosterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_booster_state(
        &mut self,
        booster_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_booster = self.u_data.items.get_mut(booster_key).dc_booster_mut().unwrap();
        let old_a_state = u_booster.get_state();
        u_booster.set_booster_state(state);
        let new_a_state = u_booster.get_state();
        u_booster.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            booster_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> BoosterMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_booster_state(self.key, state, &mut reuse_eupdates)
    }
}
