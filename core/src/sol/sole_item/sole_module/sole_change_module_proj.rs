use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn change_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeModuleProjError> {
        // Check if projection is defined before changing it
        let module = self.uad.items.get_item_mut(item_id)?.get_module_mut()?;
        let old_range = match module.get_projs().get(projectee_item_id) {
            Some(old_range) => *old_range,
            None => return Err(ProjFoundError::new(*item_id, *projectee_item_id).into()),
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update skeleton for module
        let charge_id = module.get_charge_id();
        module.get_projs_mut().add(*projectee_item_id, range);
        // Update services for module
        self.change_item_id_projection_range_in_svcs(item_id, projectee_item_id, range);
        if let Some(charge_id) = charge_id {
            // Update skeleton for charge
            let charge = self
                .uad
                .items
                .get_item_mut(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            charge.get_projs_mut().add(*projectee_item_id, range);
            // Update services for charge
            self.change_item_id_projection_range_in_svcs(&charge_id, projectee_item_id, range);
        }
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
