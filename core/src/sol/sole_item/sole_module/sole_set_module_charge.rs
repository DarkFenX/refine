use crate::{
    defs::{EItemId, SolItemId},
    err::basic::{ItemAllocError, ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolChargeInfo, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn set_module_charge(
        &mut self,
        item_id: &SolItemId,
        charge_type_id: EItemId,
    ) -> Result<SolChargeInfo, SetModuleChargeError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        // Remove old charge, if it was set
        if let Some(charge_id) = module.get_charge_id() {
            let charge_item = self.items.get_item(&charge_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                charge_item,
            );
            self.items.remove_item(&charge_id);
        }
        // Set new charge
        // Allocation can fail only if we didn't remove charge first, so if it fails - we don't need
        // to restore anything
        let charge_id = self.items.alloc_item_id()?;
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.set_charge_id(Some(charge_id));
        let fit_id = module.get_fit_id();
        let module_state = module.get_state();
        let charge_info = self.add_charge_with_id(charge_id, fit_id, charge_type_id, *item_id, module_state);
        Ok(charge_info)
    }
}

#[derive(Debug)]
pub enum SetModuleChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for SetModuleChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetModuleChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetModuleChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetModuleChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl From<ItemAllocError> for SetModuleChargeError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
