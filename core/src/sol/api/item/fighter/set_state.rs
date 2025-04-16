use itertools::Itertools;

use crate::sol::{ItemKey, SolarSystem, api::FighterMut, uad::item::MinionState};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_fighter_state(&mut self, item_key: ItemKey, state: MinionState) {
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        let old_a_state = uad_fighter.get_a_state();
        uad_fighter.set_fighter_state(state);
        let new_a_state = uad_fighter.get_a_state();
        // Update services for fighter
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            let old_a_state = uad_autocharge.get_a_state();
            uad_autocharge.set_a_state(state.into());
            // Update services for autocharge
            let new_a_state = uad_autocharge.get_a_state();
            self.change_item_key_state_in_svc(autocharge_key, old_a_state, new_a_state);
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_state(self, state: MinionState) -> Self {
        self.sol.internal_set_fighter_state(self.key, state);
        self
    }
}
