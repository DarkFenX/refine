use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
    ) -> Result<(), RemoveModuleProjError> {
        // Check if projection is defined
        let module = self.uad.items.get_item(item_id)?.get_module()?;
        if !module.get_projs().contains(projectee_item_id) {
            return Err(ProjFoundError::new(*item_id, *projectee_item_id).into());
        };
        let charge_id = module.get_charge_id();
        if let Some(charge_id) = charge_id {
            // Update services for charge
            self.remove_item_id_projection_from_svcs(&charge_id, projectee_item_id);
            // Update skeleton for charge
            self.proj_tracker.unreg_projectee(&charge_id, projectee_item_id);
            let charge = self
                .uad
                .items
                .get_item_mut(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            charge.get_projs_mut().remove(projectee_item_id);
        }
        // Update services for module
        self.remove_item_id_projection_from_svcs(item_id, projectee_item_id);
        // Update skeleton for module
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let module = self.uad.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.get_projs_mut().remove(projectee_item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjectionNotFound(ProjFoundError),
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
