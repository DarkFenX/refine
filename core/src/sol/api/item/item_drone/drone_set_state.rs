use crate::{
    misc::MinionState,
    sol::{SolarSystem, api::DroneMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_state(
        &mut self,
        drone_key: UItemKey,
        state: MinionState,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_drone = self.u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
        let old_a_state = u_drone.get_state();
        u_drone.set_drone_state(state);
        let new_a_state = u_drone.get_state();
        u_drone.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            drone_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> DroneMut<'a> {
    pub fn set_state(&mut self, state: MinionState) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_drone_state(self.key, state, &mut reuse_eupdates)
    }
}
