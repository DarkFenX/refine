use itertools::Itertools;

use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolModRack, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleError> {
        let item = self.items.get_item(item_id)?;
        let module = item.get_module()?;
        let charge_id = module.get_charge_id();
        // Remove outgoing projections for both module and charge
        let module_projs = module.get_projs().iter_items().map(|v| *v).collect_vec();
        if !module_projs.is_empty() {
            if let Some(charge_id) = charge_id {
                let charge_item = self.items.get_item(&charge_id).unwrap();
                // Use module projections, since module and charge projections should always match
                for projectee_item_id in module_projs.iter() {
                    let projectee_item = self.items.get_item(projectee_item_id).unwrap();
                    // Update services for charge
                    self.svcs.remove_item_projection(
                        &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                        charge_item,
                        projectee_item,
                    );
                    // Update skeleton for charge - don't touch data on charge itself, since charge
                    // will be removed later anyway
                    self.proj_tracker.unreg_projectee(&charge_id, projectee_item_id);
                }
            }
            for projectee_item_id in module_projs {
                // Update services for module
                let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
                self.svcs.remove_item_projection(
                    &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                    item,
                    projectee_item,
                );
                // Update skeleton for module - don't touch data on module itself, since module will
                // be removed later anyway
                self.proj_tracker.unreg_projectee(item_id, &projectee_item_id);
            }
        }
        // Remove charge
        if let Some(charge_id) = charge_id {
            // Update services for charge
            let charge_item = self.items.get_item(&charge_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                charge_item,
            );
            // Update skeleton for charge - not updating module<->charge references because both
            // will be removed
            self.items.remove_item(&charge_id);
        }
        // Remove module
        // Update services for module
        let item = self.items.get_item(item_id).unwrap();
        let module = item.get_module().unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Update skeleton for module
        let fit = self.fits.get_fit_mut(&module.get_fit_id()).unwrap();
        match module.get_rack() {
            SolModRack::High => fit.mods_high.remove(item_id),
            SolModRack::Mid => fit.mods_mid.remove(item_id),
            SolModRack::Low => fit.mods_low.remove(item_id),
        };
        self.items.remove_item(item_id);
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
