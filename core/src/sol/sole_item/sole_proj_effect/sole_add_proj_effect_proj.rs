use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn add_proj_effect_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), AddProjEffectProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(AddProjEffectProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(AddProjEffectProjError::ProjecteeNotFound)?;
        self.add_proj_effect_proj_internal(item_key, projectee_item_key)
    }
    pub(in crate::sol) fn add_proj_effect_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), AddProjEffectProjError> {
        // Check projector
        let proj_effect = self.uad.items.get(item_key).get_proj_effect()?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if proj_effect.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: proj_effect.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_item.get_item_id(),
                item_kind: projectee_item.get_name(),
            }
            .into());
        }
        // Update user data
        let proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        proj_effect.get_projs_mut().add(projectee_item_key, None);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services
        self.add_item_key_projection_to_svc(item_key, projectee_item_key, None);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddProjEffectProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotProjEffect(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[from] ProjNotFoundError),
}
