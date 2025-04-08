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
        let proj_effect = self
            .uad
            .items
            .get(item_key)
            .get_proj_effect()
            .map_err(AddProjEffectProjError::ProjectorIsNotProjEffect)?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if proj_effect.get_projs().contains(&projectee_item_key) {
            return Err(AddProjEffectProjError::ProjectionAlreadyExists(ProjNotFoundError {
                projector_item_id: proj_effect.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }));
        }
        // Check if projectee can receive projections
        if !projectee_item.can_receive_projs() {
            return Err(AddProjEffectProjError::ProjecteeCantTakeProjs(ItemReceiveProjError {
                item_id: projectee_item.get_item_id(),
                item_kind: projectee_item.get_name(),
            }));
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

#[derive(Debug)]
pub enum AddProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
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
impl From<ProjNotFoundError> for AddProjEffectProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
