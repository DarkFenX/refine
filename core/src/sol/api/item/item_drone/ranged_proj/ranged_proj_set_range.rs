use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::SolarSystem,
    uad::{UadItemKey, UadProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_proj_range(
        &mut self,
        item_key: UadItemKey,
        projectee_key: UadItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        let tgt_item_radius = self.uad.items.get(projectee_key).get_r_axt().map(|v| v.radius);
        // Check if projection is defined before changing it
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let old_uad_prange = match uad_drone.get_projs().get(&projectee_key) {
            Some(old_uad_prange) => old_uad_prange,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: uad_drone.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_key),
                });
            }
        };
        let uad_prange =
            UadProjRange::from_prange_with_radii(range, uad_drone.get_r_axt().map(|v| v.radius), tgt_item_radius);
        // Do nothing if ranges are equal
        if uad_prange == old_uad_prange {
            return Ok(());
        }
        // Update user data
        uad_drone.get_projs_mut().add(projectee_key, uad_prange);
        // Update services
        let uad_item = self.uad.items.get(item_key);
        let projectee_uad_item = self.uad.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
            uad_prange,
        );
        Ok(())
    }
}
