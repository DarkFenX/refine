use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::SolarSystem,
    ud::{UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_proj_range(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        let tgt_item_radius = self.u_data.items.get(projectee_key).get_axt().map(|v| v.radius);
        // Check if projection is defined before changing it
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_u_prange = match u_drone.get_projs().get(&projectee_key) {
            Some(old_u_prange) => old_u_prange,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: u_drone.get_item_id(),
                    projectee_item_id: self.u_data.items.id_by_key(projectee_key),
                });
            }
        };
        let u_prange = UProjRange::from_prange_with_radii(range, u_drone.get_axt().map(|v| v.radius), tgt_item_radius);
        // Do nothing if ranges are equal
        if u_prange == old_u_prange {
            return Ok(());
        }
        // Update user data
        u_drone.get_projs_mut().add(projectee_key, u_prange);
        // Update services
        let u_item = self.u_data.items.get(item_key);
        let projectee_u_item = self.u_data.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
            u_prange,
        );
        Ok(())
    }
}
