use crate::{api::ProjMut, err::basic::ProjFoundError, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_proj_effect_proj(
        &mut self,
        proj_effect_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_proj_effect = self.u_data.items.get(proj_effect_uid).dc_proj_effect().unwrap();
        if !u_proj_effect.get_projs().contains(&projectee_uid) {
            return Err(ProjFoundError {
                projector_item_id: u_proj_effect.get_item_id(),
                projectee_item_id: self.u_data.items.xid_by_iid(projectee_uid),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, proj_effect_uid, projectee_uid);
        // Update user data
        self.rev_projs.unreg_projectee(&proj_effect_uid, projectee_uid);
        let u_proj_effect = self.u_data.items.get_mut(proj_effect_uid).dc_proj_effect_mut().unwrap();
        u_proj_effect.get_projs_mut().remove(&projectee_uid);
        Ok(())
    }
}

impl<'a> ProjMut<'a> {
    pub fn remove(self) {
        self.sol
            .internal_remove_proj_effect_proj(self.projector_uid, self.projectee_uid)
            .unwrap();
    }
}
