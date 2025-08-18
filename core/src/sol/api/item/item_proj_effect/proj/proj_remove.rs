use crate::{
    err::basic::ProjFoundError,
    sol::{SolarSystem, api::ProjMut},
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect_proj(
        &mut self,
        proj_effect_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_proj_effect = self.u_data.items.get(proj_effect_key).get_proj_effect().unwrap();
        if !u_proj_effect.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_proj_effect.get_item_id(),
                projectee_item_id: self.u_data.items.id_by_key(projectee_key),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, proj_effect_key, projectee_key);
        // Update user data
        self.rev_projs.unreg_projectee(&proj_effect_key, &projectee_key);
        let u_proj_effect = self
            .u_data
            .items
            .get_mut(proj_effect_key)
            .get_proj_effect_mut()
            .unwrap();
        u_proj_effect.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}

impl<'a> ProjMut<'a> {
    pub fn remove(self) {
        self.sol
            .internal_remove_proj_effect_proj(self.projector_key, self.projectee_key)
            .unwrap();
    }
}
