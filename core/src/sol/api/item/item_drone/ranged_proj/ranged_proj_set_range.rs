use crate::{
    def::{AttrVal, ItemKey},
    err::basic::ProjFoundError,
    sol::SolarSystem,
    uad::ProjRange,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_proj_range(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_range = match uad_drone.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: uad_drone.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                });
            }
        };
        // Do nothing if ranges are equal
        if range == old_range.map(|v| v.c2c) {
            return Ok(());
        }
        // Update user data
        uad_drone
            .get_projs_mut()
            .add(projectee_item_key, range.map(ProjRange::new_tmp));
        // Update services
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            projectee_item_key,
            projectee_uad_item,
            range.map(ProjRange::new_tmp),
        );
        Ok(())
    }
}
