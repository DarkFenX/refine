use crate::{
    api::DroneMut,
    misc::NpcProp,
    sol::SolarSystem,
    ud::{UItemId, UNpcProp},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_prop_mode(&mut self, drone_key: UItemId, prop_mode: UNpcProp) {
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        u_drone.set_npc_prop(prop_mode);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone propulsion mode.
    pub fn set_prop_mode(&mut self, prop_mode: NpcProp) {
        self.sol.internal_set_drone_prop_mode(self.key, prop_mode.into());
    }
}
