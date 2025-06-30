use crate::{
    def::ItemKey,
    misc::MinionState,
    sol::{SolarSystem, api::DroneMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_state(&mut self, item_key: ItemKey, state: MinionState) {
        let drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_a_state = drone.get_a_state();
        drone.set_drone_state(state);
        let new_a_state = drone.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
    }
}

impl<'a> DroneMut<'a> {
    pub fn set_state(&mut self, state: MinionState) {
        self.sol.internal_set_drone_state(self.key, state)
    }
}
