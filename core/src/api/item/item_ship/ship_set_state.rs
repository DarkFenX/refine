use crate::{
    api::ShipMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_ship_state(
        &mut self,
        ship_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_ship = self.u_data.items.get_mut(ship_key).dc_ship_mut().unwrap();
        let old_a_state = u_ship.get_state();
        u_ship.set_ship_state(state);
        let new_a_state = u_ship.get_state();
        u_ship.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            ship_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ShipMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_ship_state(self.key, state, &mut reuse_eupdates)
    }
}
