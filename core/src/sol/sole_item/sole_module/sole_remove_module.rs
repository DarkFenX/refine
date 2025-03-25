use itertools::Itertools;

use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

use super::{misc::get_fit_rack, pos_modes::RmMode};

impl SolarSystem {
    pub fn remove_module(&mut self, item_id: &ItemId, pos_mode: RmMode) -> Result<(), RemoveModuleError> {
        let item = self.uad.items.get_item(item_id)?;
        let module = item.get_module()?;
        let fit_id = module.get_fit_id();
        let rack = module.get_rack();
        let charge_id = module.get_charge_item_id();
        // Remove outgoing projections for both module and charge
        let module_projs = module.get_projs().iter_items().copied().collect_vec();
        if !module_projs.is_empty() {
            if let Some(charge_id) = charge_id {
                let charge_item = self.uad.items.get_item(&charge_id).unwrap();
                // Use module projections, since module and charge projections should always match
                for projectee_item_id in module_projs.iter() {
                    let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
                    // Update services for charge
                    self.svc.remove_item_projection(&self.uad, charge_item, projectee_item);
                    // Update user data for charge - don't touch data on charge itself, since charge
                    // will be removed later anyway
                    self.proj_tracker.unreg_projectee(&charge_id, projectee_item_id);
                }
            }
            for projectee_item_id in module_projs {
                // Update services for module
                let projectee_item = self.uad.items.get_item(&projectee_item_id).unwrap();
                self.svc.remove_item_projection(&self.uad, item, projectee_item);
                // Update user data for module - don't touch data on module itself, since module
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(item_id, &projectee_item_id);
            }
        }
        // Remove charge
        if let Some(charge_id) = charge_id {
            // Update services for charge
            let charge_item = self.uad.items.get_item(&charge_id).unwrap();
            self.svc.remove_item(&self.uad, charge_item);
            // Update user data for charge - not updating module<->charge references because both
            // will be removed
            self.uad.items.remove_item(&charge_id);
        }
        // Remove module
        // Update services for module
        self.remove_item_id_from_svc(item_id);
        // Update user data for module
        let fit_rack = get_fit_rack(&mut self.uad.fits, &fit_id, rack).unwrap();
        match pos_mode {
            RmMode::Free => fit_rack.free(item_id),
            RmMode::Remove => {
                if let Some(pos) = fit_rack.remove(item_id) {
                    for (i, rack_module_id) in fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_id) = rack_module_id {
                            self.uad
                                .items
                                .get_item_mut(rack_module_id)
                                .unwrap()
                                .get_module_mut()
                                .unwrap()
                                .set_pos(pos + i);
                        }
                    }
                }
            }
        }
        self.uad.items.remove_item(item_id);
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
