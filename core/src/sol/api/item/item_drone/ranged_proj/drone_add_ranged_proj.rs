use crate::{
    def::ItemId,
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    misc::ProjRange,
    sol::{
        SolarSystem,
        api::{AddRangedProjError, DroneMut, RangedProjMut},
    },
    ud::{UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let u_drone = self.u_data.items.get(item_key).get_drone().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_drone.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_drone.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_u_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        let u_prange = UProjRange::from_prange_with_axt(range, u_drone.get_axt(), projectee_u_item.get_axt());
        // Update user data
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        u_drone.get_projs_mut().add(projectee_key, u_prange);
        self.rev_projs.reg_projectee(item_key, projectee_key);
        // Update services
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, item_key, projectee_key, u_prange);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn add_proj(
        &mut self,
        projectee_item_id: &ItemId,
        range: ProjRange,
    ) -> Result<RangedProjMut<'_>, AddRangedProjError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_drone_proj(self.key, projectee_key, range)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
