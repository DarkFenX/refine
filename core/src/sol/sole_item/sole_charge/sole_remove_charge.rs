use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_charge(&mut self, item_id: &ItemId) -> Result<(), RemoveChargeError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_charge_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_charge_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let charge = item.get_charge()?;
        // Remove outgoing projections
        for &projectee_item_key in charge.get_projs().iter_projectee_item_keys() {
            // Update services for charge
            let projectee_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
            // Update user data for charge - do not touch projections container on charge itself,
            // because we're removing it anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Update services
        self.svc.remove_item(&self.uad, item_key, item);
        // Update user data
        let module_item_key = charge.get_cont_item_key();
        let module = self.uad.items.get_mut(module_item_key).get_module_mut().unwrap();
        module.set_charge_item_key(None);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
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
