use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_charge(&mut self, item_id: &SolItemId) -> Result<(), RemoveChargeError> {
        let item = self.items.get_item(item_id)?;
        let charge = item.get_charge()?;
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        let module_item_id = charge.cont_id;
        let module = self
            .items
            .get_item_mut(&module_item_id)
            .unwrap()
            .get_module_mut()
            .unwrap();
        module.charge_item_id = None;
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
}
impl From<ItemFoundError> for RemoveChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharge(error)
    }
}
impl std::error::Error for RemoveChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharge(e) => e.fmt(f),
        }
    }
}
