use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn change_module_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeModuleProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(ChangeModuleProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(ChangeModuleProjError::ProjecteeNotFound)?;
        self.change_module_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn change_module_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeModuleProjError> {
        // Check if projection is defined before changing it
        let module = self.uad.items.get_mut(item_key).get_module_mut()?;
        let old_range = match module.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: module.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                }
                .into());
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data for module
        let charge_key = module.get_charge_item_key();
        module.get_projs_mut().add(projectee_item_key, range);
        // Update services for module
        self.change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            charge.get_projs_mut().add(projectee_item_key, range);
            // Update services for charge
            self.change_item_key_projection_range_in_svc(charge_key, projectee_item_key, range);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeModuleProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotModule(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
