use crate::{
    defs::{EItemId, SolItemId},
    err::basic::{ItemAllocError, ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolChargeInfo, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn set_module_charge(
        &mut self,
        item_id: &SolItemId,
        charge_a_item_id: EItemId,
    ) -> Result<SolChargeInfo, AddSetModuleChargeError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        // Remove old charge, if it was set
        if let Some(charge_item_id) = module.charge_item_id {
            let charge_item = self.items.get_item(&charge_item_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                charge_item,
            );
            self.items.remove_item(&charge_item_id);
        }
        // Set new charge
        let charge_item_id = self.items.alloc_item_id()?;
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.charge_item_id = Some(charge_item_id);
        let fit_id = module.get_fit_id();
        let charge_info = self.add_charge_with_id(charge_item_id, fit_id, charge_a_item_id, *item_id);
        Ok(charge_info)
    }
}

#[derive(Debug)]
pub enum AddSetModuleChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<ItemFoundError> for AddSetModuleChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for AddSetModuleChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl From<ItemAllocError> for AddSetModuleChargeError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddSetModuleChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddSetModuleChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
