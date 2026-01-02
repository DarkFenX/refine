use crate::{
    api::{AddProjError, DroneMut, RangedProjMut},
    def::ItemId,
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
    ud::{UItemId, UProjData},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_drone_proj(
        &mut self,
        drone_key: UItemId,
        projectee_key: UItemId,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_drone = self.u_data.items.get(drone_key).dc_drone().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_drone.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_drone.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections by getting its user physics
        let projectee_physics = match projectee_u_item.get_direct_physics() {
            Some(projectee_physics) => *projectee_physics,
            None => {
                return Err(ItemReceiveProjError {
                    item_id: projectee_u_item.get_item_id(),
                    item_kind: projectee_u_item.get_name(),
                }
                .into());
            }
        };
        let drone_physics = *u_drone.get_physics();
        let u_proj_data = Some(UProjData::from_physics_with_axt(
            drone_physics,
            projectee_physics,
            u_drone.get_axt(),
            projectee_u_item.get_axt(),
        ));
        // Update user data
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        u_drone.get_projs_mut().add(projectee_key, u_proj_data);
        self.rev_projs.reg_projectee(drone_key, projectee_key);
        // Update services
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, drone_key, projectee_key, u_proj_data);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProjMut<'_>, AddProjError> {
        let projectee_key = self.sol.u_data.items.iid_by_eid_err(projectee_item_id)?;
        self.sol.internal_add_drone_proj(self.key, projectee_key)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
