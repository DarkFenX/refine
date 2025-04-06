use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_proj_effect_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveProjEffectProjError> {
        // Check if projection is defined
        let proj_effect_item = self.uad.items.get_by_id(item_id)?;
        let proj_effect = proj_effect_item.get_proj_effect()?;
        if !proj_effect.get_projs().contains(projectee_item_id) {
            return Err(ProjFoundError {
                projector_item_id: *item_id,
                projectee_item_id: *projectee_item_id,
            }
            .into());
        };
        // Update services
        let projectee_item = self.uad.items.get_by_id(projectee_item_id).unwrap();
        self.svc
            .remove_item_projection(&self.uad, proj_effect_item, projectee_item);
        // Update user data
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let proj_effect = self
            .uad
            .items
            .get_mut_by_id(item_id)
            .unwrap()
            .get_proj_effect_mut()
            .unwrap();
        proj_effect.get_projs_mut().remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveProjEffectProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotProjEffect(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveProjEffectProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotProjEffect(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveProjEffectProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
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
