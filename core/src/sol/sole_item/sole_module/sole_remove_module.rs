use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

use super::{pos_modes::RmMode, shared::get_fit_rack};

impl SolarSystem {
    pub fn remove_module(&mut self, item_id: &ItemId, pos_mode: RmMode) -> Result<(), RemoveModuleError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_module_internal(item_key, pos_mode)?)
    }
    pub(in crate::sol) fn remove_module_internal(
        &mut self,
        item_key: ItemKey,
        pos_mode: RmMode,
    ) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let module = item.get_module()?;
        let fit_id = module.get_fit_id();
        let rack = module.get_rack();
        let charge_key = module.get_charge_item_key();
        // Remove outgoing projections for both module and charge
        let module_projectee_item_keys = module.get_projs().iter_projectee_item_keys().copied().collect_vec();
        if !module_projectee_item_keys.is_empty() {
            if let Some(charge_key) = charge_key {
                // Use module projections, since module and charge projections should always match
                for &projectee_item_key in module_projectee_item_keys.iter() {
                    let projectee_item = self.uad.items.get(projectee_item_key);
                    // Update services for charge
                    self.svc
                        .remove_item_projection(&self.uad, charge_key, projectee_item_key, projectee_item);
                    // Update user data for charge - don't touch data on charge itself, since charge
                    // will be removed later anyway
                    self.proj_tracker.unreg_projectee(&charge_key, &projectee_item_key);
                }
            }
            for projectee_item_id in module_projectee_item_keys {
                // Update services for module
                let projectee_item = self.uad.items.get(projectee_item_id);
                self.svc
                    .remove_item_projection(&self.uad, item_key, projectee_item_id, projectee_item);
                // Update user data for module - don't touch data on module itself, since module
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(&item_key, &projectee_item_id);
            }
        }
        // Remove charge
        if let Some(charge_key) = charge_key {
            // Update services for charge
            let charge_item = self.uad.items.get(charge_key);
            self.svc.remove_item(&self.uad, charge_key, charge_item);
            // Update user data for charge - not updating module<->charge references because both
            // will be removed
            self.uad.items.remove(charge_key);
        }
        // Remove module
        // Update services for module
        self.remove_item_key_from_svc(item_key);
        // Update user data for module
        let fit_rack = get_fit_rack(&mut self.uad.fits, &fit_id, rack).unwrap();
        match pos_mode {
            RmMode::Free => fit_rack.free(&item_key),
            RmMode::Remove => {
                if let Some(pos) = fit_rack.remove(&item_key) {
                    for (i, rack_module_key) in fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.uad
                                .items
                                .get_mut(*rack_module_key)
                                .get_module_mut()
                                .unwrap()
                                .set_pos(pos + i);
                        }
                    }
                }
            }
        }
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
}
impl std::error::Error for RemoveModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveModuleError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveModuleError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
