use crate::sol::{ItemKey, SolarSystem, api::AutochargeMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_autocharge_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_autocharge = self.uad.items.get_mut(item_key).get_autocharge_mut().unwrap();
        let old_a_state = uad_autocharge.get_a_state();
        uad_autocharge.set_force_disable(!state);
        let new_a_state = uad_autocharge.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> AutochargeMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_autocharge_state(self.key, state)
    }
}
