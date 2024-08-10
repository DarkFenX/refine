use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ChargeFoundError, ItemFoundError, ItemKindMatchError},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn remove_module_charge(&mut self, item_id: &SolItemId) -> Result<(), RemoveModuleChargeError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        let charge_item_id = match module.charge_item_id {
            Some(charge_item_id) => charge_item_id,
            None => return Err(ChargeFoundError::new(*item_id).into()),
        };
        self.remove_charge(&charge_item_id).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    ChargeNotSet(ChargeFoundError),
}
impl From<ItemFoundError> for RemoveModuleChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveModuleChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl From<ChargeFoundError> for RemoveModuleChargeError {
    fn from(error: ChargeFoundError) -> Self {
        Self::ChargeNotSet(error)
    }
}
impl std::error::Error for RemoveModuleChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
            Self::ChargeNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveModuleChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
            Self::ChargeNotSet(e) => e.fmt(f),
        }
    }
}
