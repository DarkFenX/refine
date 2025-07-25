use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemKey};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_item = self.u_data.items.get(item_key);
        let u_drone = u_item.get_drone().unwrap();
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if !u_drone.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_drone.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
        );
        // Update user data
        self.rev_projs.unreg_projectee(&item_key, &projectee_key);
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        u_drone.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
