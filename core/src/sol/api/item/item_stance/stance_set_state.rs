use crate::sol::{ItemKey, SolarSystem, api::StanceMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_stance_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_stance = self.uad.items.get_mut(item_key).get_stance_mut().unwrap();
        let old_a_state = uad_stance.get_a_state();
        uad_stance.set_stance_state(state);
        let new_a_state = uad_stance.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> StanceMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_stance_state(self.key, state)
    }
}
