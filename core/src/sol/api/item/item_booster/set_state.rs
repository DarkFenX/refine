use crate::sol::{ItemKey, SolarSystem, api::BoosterMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_booster_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_booster = self.uad.items.get_mut(item_key).get_booster_mut().unwrap();
        let old_a_state = uad_booster.get_a_state();
        uad_booster.set_booster_state(state);
        let new_a_state = uad_booster.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_booster_state(self.key, state)
    }
}
