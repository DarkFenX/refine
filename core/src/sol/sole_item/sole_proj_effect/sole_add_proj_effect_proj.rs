use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn add_proj_effect_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
    ) -> Result<(), AddProjEffectProjError> {
        // Check projector
        let proj_effect = self
            .items
            .get_item(item_id)
            .map_err(|e| AddProjEffectProjError::ProjectorNotFound(e))?
            .get_proj_effect()
            .map_err(|e| AddProjEffectProjError::ProjectorIsNotProjEffect(e))?;
        // Check if projection has already been defined
        if proj_effect.projs.contains(&projectee_item_id) {
            return Err(AddProjEffectProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .items
            .get_item(&projectee_item_id)
            .map_err(|e| AddProjEffectProjError::ProjecteeNotFound(e))?;
        if !projectee_item.can_receive_projs() {
            return Err(AddProjEffectProjError::ProjecteeCantTakeProjs(
                ItemReceiveProjError::new(projectee_item_id, projectee_item.get_name()),
            ));
        }
        // Update skeleton
        let proj_effect = self.items.get_item_mut(item_id).unwrap().get_proj_effect_mut().unwrap();
        proj_effect.projs.add(projectee_item_id, None);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
        self.svcs.add_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
            None,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl From<ProjNotFoundError> for AddProjEffectProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
impl std::error::Error for AddProjEffectProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotProjEffect(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddProjEffectProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotProjEffect(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
