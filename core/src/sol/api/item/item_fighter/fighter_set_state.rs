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
        let old_state = u_fighter.get_state();
        u_fighter.set_fighter_state(state, reuse_eupdates, &self.u_data.src);
        let new_state = u_fighter.get_state();
        // Filter out autocharges which couldn't be loaded, and fill autocharge key data
        let ac_activations = reuse_eupdates
            .autocharges
            .iter()
            .filter_map(|ac_act| {
                u_fighter
                    .get_autocharges()
                    .get_ac_key(&ac_act.effect_key)
                    .map(|ac_key| (ac_key, ac_act.active))
            })
            .collect_vec();
        // Update services for fighter
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            item_key,
            old_state,
            new_state,
            reuse_eupdates,
        );
        SolarSystem::util_process_autocharge_activations(
            &mut self.u_data,
            &mut self.svc,
            ac_activations,
            reuse_eupdates,
        );
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_state(&mut self, state: MinionState) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fighter_state(self.key, state, &mut reuse_eupdates)
    }
}
