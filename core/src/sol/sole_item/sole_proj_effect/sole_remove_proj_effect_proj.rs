use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_proj_effect_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
    ) -> Result<(), RemoveProjEffectProjError> {
        // Check if projection is defined
        let proj_effect = self.items.get_item(item_id)?.get_proj_effect()?;
        if !proj_effect.projs.contains(projectee_item_id) {
            return Err(ProjFoundError::new(*item_id, *projectee_item_id).into());
        };
        // Process request in services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(projectee_item_id).unwrap();
        self.svcs.remove_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
        );
        // Update the skeleton
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let proj_effect = self.items.get_item_mut(item_id).unwrap().get_proj_effect_mut().unwrap();
        proj_effect.projs.remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
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
