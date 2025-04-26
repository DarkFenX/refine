use crate::sol::{ItemKey, SolarSystem, api::ShipMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_ship_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        let old_a_state = uad_ship.get_a_state();
        uad_ship.set_ship_state(state);
        let new_a_state = uad_ship.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> ShipMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_ship_state(self.key, state)
    }
}
