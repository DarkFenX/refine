use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) -> Result<(), RemoveModuleProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(RemoveModuleProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(RemoveModuleProjError::ProjecteeNotFound)?;
        self.remove_module_proj_internal(item_key, projectee_item_key)
    }
    pub(in crate::sol) fn remove_module_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), RemoveModuleProjError> {
        // Check if projection is defined
        let module = self.uad.items.get(item_key).get_module()?;
        if !module.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: module.get_item_id(),
                projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
            }
            .into());
        };
        let charge_key = module.get_charge_item_key();
        if let Some(charge_key) = charge_key {
            // Update services for charge
            self.remove_item_key_projection_from_svc(charge_key, projectee_item_key);
            // Update user data for charge
            self.proj_tracker.unreg_projectee(&charge_key, &projectee_item_key);
            let charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            charge.get_projs_mut().remove(&projectee_item_key);
        }
        // Update services for module
        self.remove_item_key_projection_from_svc(item_key, projectee_item_key);
        // Update user data for module
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        module.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjectionNotFound(ProjFoundError),
}
impl std::error::Error for RemoveModuleProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotModule(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjectionNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveModuleProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotModule(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjectionNotFound(e) => e.fmt(f),
        }
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
