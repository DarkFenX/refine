use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_proj_effect_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveProjEffectProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(RemoveProjEffectProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(RemoveProjEffectProjError::ProjecteeNotFound)?;
        self.remove_proj_effect_proj_internal(item_key, projectee_item_key)
    }
    pub(in crate::sol) fn remove_proj_effect_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), RemoveProjEffectProjError> {
        // Check if projection is defined
        let proj_effect = self.uad.items.get(item_key).get_proj_effect()?;
        let projectee_item = self.uad.items.get(projectee_item_key);
        if !proj_effect.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: proj_effect.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }
            .into());
        };
        // Update services
        self.svc
            .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
        // Update user data
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        proj_effect.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveProjEffectProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotProjEffect(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
