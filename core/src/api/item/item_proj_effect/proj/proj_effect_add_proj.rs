use crate::{
    api::{AddProjError, ProjEffectMut, ProjMut},
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
    ud::{ItemId, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_proj_effect_proj(
        &mut self,
        proj_effect_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_item = self.u_data.items.get(proj_effect_uid);
        let u_proj_effect = u_item.dc_proj_effect().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_uid);
        if u_proj_effect.get_projs().contains(&projectee_uid) {
            return Err(ProjNotFoundError {
                projector_item_id: u_proj_effect.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections by checking if item type supports user physics
        if projectee_u_item.get_direct_physics().is_none() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.lib_get_name(),
            }
            .into());
        }
        // Update user data
        let u_proj_effect = self.u_data.items.get_mut(proj_effect_uid).dc_proj_effect_mut().unwrap();
        u_proj_effect.get_projs_mut().add(projectee_uid, None);
        self.rev_projs.reg_projectee(proj_effect_uid, projectee_uid);
        // Update services
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, proj_effect_uid, projectee_uid, None);
        Ok(())
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, AddProjError> {
        let projectee_uid = self.sol.u_data.items.iid_by_xid_err(projectee_item_id)?;
        self.sol.internal_add_proj_effect_proj(self.uid, projectee_uid)?;
        Ok(ProjMut::new(self.sol, self.uid, projectee_uid))
    }
}
