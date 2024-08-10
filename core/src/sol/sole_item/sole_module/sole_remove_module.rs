use itertools::Itertools;

use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{SolModRack, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleError> {
        let item = self.items.get_item(item_id)?;
        let module = item.get_module()?;
        let charge_item_id = module.charge_item_id;
        // Remove outgoing projections
        let proj_outgoing = module.projs.iter_items().map(|v| *v).collect_vec();
        for projectee_item_id in proj_outgoing {
            self.remove_module_proj(item_id, &projectee_item_id).unwrap();
        }
        // Remove charge - simplified process because both will be discarded
        if let Some(charge_item_id) = charge_item_id {
            let charge_item = self.items.get_item(&charge_item_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                charge_item,
            );
            self.items.remove_item(&charge_item_id);
        }
        // Remove module from services
        let item = self.items.get_item(item_id).unwrap();
        let module = item.get_module().unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Remove module from skeleton
        let fit = self.fits.get_fit_mut(&module.get_fit_id()).unwrap();
        match module.rack {
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
