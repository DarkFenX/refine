use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemKey};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_proj(
        &mut self,
        drone_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_drone = self.u_data.items.get(drone_key).get_drone().unwrap();
        if !u_drone.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_drone.get_item_id(),
                projectee_item_id: self.u_data.items.id_by_key(projectee_key),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, drone_key, projectee_key);
        // Update user data
        self.rev_projs.unreg_projectee(&drone_key, projectee_key);
        let u_drone = self.u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
        u_drone.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
