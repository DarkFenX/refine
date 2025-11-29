use crate::{
    misc::NpcProp,
    sol::{SolarSystem, api::DroneMut},
    ud::{UItemKey, UNpcProp},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_prop_mode(&mut self, drone_key: UItemKey, prop_mode: UNpcProp) {
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        u_drone.set_prop_mode(prop_mode);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone propulsion mode.
    pub fn set_prop_mode(&mut self, prop_mode: NpcProp) {
        self.sol.internal_set_drone_prop_mode(self.key, prop_mode.into());
    }
}
