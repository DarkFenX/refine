use crate::{
    defs::{AttrVal, SolItemId},
    err::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn add_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddModuleProjError> {
        // Check projector
        let module = self
            .items
            .get_item(item_id)
            .map_err(|e| AddModuleProjError::ProjectorNotFound(e))?
            .get_module()
            .map_err(|e| AddModuleProjError::ProjectorIsNotModule(e))?;
        // Check if projection has already been defined
        if module.projs.contains(&projectee_item_id) {
            return Err(AddModuleProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .items
            .get_item(&projectee_item_id)
            .map_err(|e| AddModuleProjError::ProjecteeNotFound(e))?;
        if !projectee_item.can_receive_projs() {
            return Err(AddModuleProjError::ProjecteeCantTakeProjs(ItemReceiveProjError::new(
                projectee_item_id,
                projectee_item.get_name(),
            )));
        }
        // Update skeleton
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.projs.add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
        self.svcs.add_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
            range,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl From<ProjNotFoundError> for AddModuleProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
impl std::error::Error for AddModuleProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotModule(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotModule(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
