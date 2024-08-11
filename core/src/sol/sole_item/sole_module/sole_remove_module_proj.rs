use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
    ) -> Result<(), RemoveModuleProjError> {
        // Check if projection is defined
        let module = self.items.get_item(item_id)?.get_module()?;
        if !module.projs.contains(projectee_item_id) {
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
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.projs.remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl From<ItemFoundError> for RemoveModuleProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveModuleProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotModule(error)
    }
}
impl From<ProjFoundError> for RemoveModuleProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
impl std::error::Error for RemoveModuleProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotModule(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveModuleProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotModule(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
