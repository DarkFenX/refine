use crate::{
    sol::{SolarSystem, api::ShipMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_state(
        &mut self,
        item_key: UadItemKey,
        state: bool,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        let old_a_state = uad_ship.get_a_state();
        uad_ship.set_ship_state(state, reuse_eupdates, &self.uad.src);
        let new_a_state = uad_ship.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ShipMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_set_ship_state(self.key, state, &mut reuse_eupdates)
    }
}
