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
        let proj_effect_item = self.uad.items.get(item_key);
        let proj_effect = proj_effect_item.get_proj_effect()?;
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

#[derive(Debug)]
pub enum RemoveProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveProjEffectProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotProjEffect(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveProjEffectProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotProjEffect(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemKindMatchError> for RemoveProjEffectProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotProjEffect(error)
    }
}
impl From<ProjFoundError> for RemoveProjEffectProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
