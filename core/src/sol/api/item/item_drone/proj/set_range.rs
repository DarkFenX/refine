use crate::{
    err::basic::ProjFoundError,
    sol::{AttrVal, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_proj_range(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_range = match drone.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: drone.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                });
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data
        drone.get_projs_mut().add(projectee_item_key, range);
        // Update services
        self.internal_change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        Ok(())
    }
}
