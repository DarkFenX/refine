use crate::{
    api::DroneMut,
    misc::{NpcProp, StOption},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_prop_mode(&mut self, drone_uid: UItemId, npc_prop: StOption<NpcProp>) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone.set_npc_prop(npc_prop);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone propulsion mode.
    pub fn set_prop_mode(&mut self, prop_mode: StOption<NpcProp>) {
        self.sol.internal_set_drone_prop_mode(self.uid, prop_mode);
    }
}
