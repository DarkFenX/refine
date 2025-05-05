use crate::{
    err::basic::ProjFoundError,
    sol::{ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_drone = self.uad.items.get(item_key).get_drone().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if !uad_drone.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_drone.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            });
        };
        // Update services
        self.svc
            .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
        // Update user data
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}
