use crate::{
    err::basic::{ItemFoundError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem, api::ProjEffectMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_proj_effect_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), RemoveProjEffectProjError> {
        // Check if projection is defined
        let uad_proj_effect = self.uad.items.get(item_key).get_proj_effect().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if !uad_proj_effect.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_proj_effect.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            }
            .into());
        };
        // Update services
        self.svc
            .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
        // Update user data
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let uad_proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        uad_proj_effect.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn remove_proj(&mut self, projectee_item_id: &ItemId) -> Result<(), RemoveProjEffectProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol
            .internal_remove_proj_effect_proj(self.key, projectee_item_key)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveProjEffectProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
