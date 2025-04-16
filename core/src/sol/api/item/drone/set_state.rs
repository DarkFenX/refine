use crate::sol::{ItemKey, SolarSystem, api::DroneMut, uad::item::MinionState};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_drone_state(&mut self, item_key: ItemKey, state: MinionState) {
        let drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_a_state = drone.get_a_state();
        drone.set_drone_state(state);
        let new_a_state = drone.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> DroneMut<'a> {
    pub fn set_state(self, state: MinionState) -> Self {
        self.sol.internal_set_drone_state(self.key, state);
        self
    }
}
