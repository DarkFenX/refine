use crate::{
    err::basic::{ItemFoundError, ProjFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem, api::ModuleMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_module_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let old_range = match uad_module.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: uad_module.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                });
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data for module
        let charge_key = uad_module.get_charge_item_key();
        uad_module.get_projs_mut().add(projectee_item_key, range);
        // Update services for module
        self.change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            uad_charge.get_projs_mut().add(projectee_item_key, range);
            // Update services for charge
            self.change_item_key_projection_range_in_svc(charge_key, projectee_item_key, range);
        }
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn change_proj_range(
        self,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<Self, ChangeModuleProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol
            .internal_change_module_proj(self.key, projectee_item_key, range)?;
        Ok(self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeModuleProjError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
