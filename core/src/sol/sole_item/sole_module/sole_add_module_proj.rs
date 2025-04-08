use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn add_module_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddModuleProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(AddModuleProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(AddModuleProjError::ProjecteeNotFound)?;
        self.add_module_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn add_module_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), AddModuleProjError> {
        // Check projector
        let module = self
            .uad
            .items
            .get(item_key)
            .get_module()
            .map_err(AddModuleProjError::ProjectorIsNotModule)?;
        // Check if projection has already been defined
        let projectee_item = self.uad.items.get(projectee_item_key);
        if module.get_projs().contains(&projectee_item_key) {
            return Err(AddModuleProjError::ProjectionAlreadyExists(ProjNotFoundError {
                projector_item_id: module.get_item_id(),
                projectee_item_id: projectee_item.get_item_id(),
            }));
        }
        // Check if projectee can receive projections
        if !projectee_item.can_receive_projs() {
            return Err(AddModuleProjError::ProjecteeCantTakeProjs(ItemReceiveProjError {
                item_id: projectee_item.get_item_id(),
                item_kind: projectee_item.get_name(),
            }));
        }
        // Update user data for module
        let module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = module.get_charge_item_key();
        module.get_projs_mut().add(projectee_item_key, range);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services for module
        self.add_item_key_projection_to_svc(item_key, projectee_item_key, range);
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            charge.get_projs_mut().add(projectee_item_key, range);
            self.proj_tracker.reg_projectee(charge_key, projectee_item_key);
            // Update services for charge
            self.add_item_key_projection_to_svc(charge_key, projectee_item_key, range);
        }
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
impl From<ProjNotFoundError> for AddModuleProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
