use crate::{api::DroneMut, misc::NpcProp, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_npc_prop(&mut self, drone_uid: UItemId, npc_prop: Option<NpcProp>) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone.set_npc_prop(npc_prop);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone propulsion mode.
    pub fn set_npc_prop(&mut self, npc_prop: Option<NpcProp>) {
        self.sol.internal_set_drone_npc_prop(self.uid, npc_prop);
    }
}
