use crate::{
    def::{ItemId, ItemKey},
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    misc::ProjRange,
    sol::{
        SolarSystem,
        api::{AddRangedProjError, DroneMut, RangedProjMut},
    },
    uad::UadProjRange,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone_proj(
        &mut self,
        item_key: ItemKey,
        projectee_key: ItemKey,
        range: ProjRange,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_key);
        if uad_drone.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: uad_drone.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_uad_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_uad_item.get_item_id(),
                item_kind: projectee_uad_item.get_name(),
            }
            .into());
        }
        let uad_prange = UadProjRange::from_prange_with_xt(range, uad_drone.get_a_xt(), projectee_uad_item.get_a_xt());
        // Update services
        SolarSystem::util_add_item_projection(
            &self.uad,
            &mut self.svc,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
            uad_prange,
        );
        // Update user data
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.get_projs_mut().add(projectee_key, uad_prange);
        self.rprojs.reg_projectee(item_key, projectee_key);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn add_proj(
        &mut self,
        projectee_item_id: &ItemId,
        range: ProjRange,
    ) -> Result<RangedProjMut<'_>, AddRangedProjError> {
        let projectee_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_drone_proj(self.key, projectee_key, range)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
