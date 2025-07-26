use itertools::Itertools;

use crate::{
    misc::MinionState,
    sol::{SolarSystem, api::FighterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_state(
        &mut self,
        item_key: UItemKey,
        state: MinionState,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Update user data for fighter
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        let autocharge_keys = u_fighter.get_autocharges().values().copied().collect_vec();
        let old_a_state = u_fighter.get_state();
        u_fighter.set_fighter_state(state, reuse_eupdates, &self.u_data.src);
        let new_a_state = u_fighter.get_state();
        // Update services for fighter
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let u_autocharge = self.u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            let old_a_state = u_autocharge.get_state();
            u_autocharge.set_state(state.into(), reuse_eupdates, &self.u_data.src);
            // Update services for autocharge
            let new_a_state = u_autocharge.get_state();
            SolarSystem::util_switch_item_state(
                &self.u_data,
                &mut self.svc,
                autocharge_key,
                old_a_state,
                new_a_state,
                reuse_eupdates,
            );
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_state(&mut self, state: MinionState) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fighter_state(self.key, state, &mut reuse_eupdates)
    }
}
