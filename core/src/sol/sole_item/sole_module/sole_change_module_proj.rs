use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn change_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeModuleProjError> {
        // Check if projection is defined before changing it
        let module = self.items.get_item(item_id)?.get_module()?;
        let old_range = match module.projs.get(projectee_item_id) {
            Some(old_range) => *old_range,
            None => return Err(ProjFoundError::new(*item_id, *projectee_item_id).into()),
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Adjust skeleton
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.projs.add(*projectee_item_id, range);
        // Process request in services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(projectee_item_id).unwrap();
        self.svcs.change_item_proj_range(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
            range,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for ChangeModuleProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotModule(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ChangeModuleProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotModule(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for ChangeModuleProjError {
    fn from(error: ItemFoundError) -> Self {
        Self::ProjectorNotFound(error)
    }
}
impl From<ItemKindMatchError> for ChangeModuleProjError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ProjectorIsNotModule(error)
    }
}
impl From<ProjFoundError> for ChangeModuleProjError {
    fn from(error: ProjFoundError) -> Self {
        Self::ProjectionNotFound(error)
    }
}
