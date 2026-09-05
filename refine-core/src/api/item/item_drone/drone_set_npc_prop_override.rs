use crate::{api::DroneMut, misc::NpcProp, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_npc_prop_override(
        &mut self,
        drone_uid: UItemId,
        npc_prop_override: Option<NpcProp>,
    ) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone.set_npc_prop_override(npc_prop_override);
    }
}

impl<'s> DroneMut<'s> {
    /// Force drone to use specific propulsion mode.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_npc_prop_override(&mut self, npc_prop_override: Option<NpcProp>) {
        self.sol
            .internal_set_drone_npc_prop_override(self.uid, npc_prop_override);
    }
}
