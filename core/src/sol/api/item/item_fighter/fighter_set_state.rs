use itertools::Itertools;

use crate::{
    misc::MinionState,
    sol::{SolarSystem, api::FighterMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_state(
        &mut self,
        item_key: UadItemKey,
        state: MinionState,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        let old_a_state = uad_fighter.get_a_state();
        uad_fighter.set_fighter_state(state, reuse_eupdates, &self.uad.src);
        let new_a_state = uad_fighter.get_a_state();
        // Update services for fighter
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            let old_a_state = uad_autocharge.get_a_state();
            uad_autocharge.set_a_state(state.into(), reuse_eupdates, &self.uad.src);
            // Update services for autocharge
            let new_a_state = uad_autocharge.get_a_state();
            SolarSystem::util_switch_item_state(
                &self.uad,
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
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_set_fighter_state(self.key, state, &mut reuse_eupdates)
    }
}
