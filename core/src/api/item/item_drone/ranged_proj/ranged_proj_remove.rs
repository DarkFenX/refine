use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_drone_proj(
        &mut self,
        drone_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_drone = self.u_data.items.get(drone_uid).dc_drone().unwrap();
        if !u_drone.get_projs().contains(&projectee_uid) {
            return Err(ProjFoundError {
                projector_item_id: u_drone.get_item_id(),
                projectee_item_id: self.u_data.items.xid_by_iid(projectee_uid),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, drone_uid, projectee_uid);
        // Update user data
        self.rev_projs.unreg_projectee(&drone_uid, projectee_uid);
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        u_drone.get_projs_mut().remove(&projectee_uid);
        Ok(())
    }
}
