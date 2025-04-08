use crate::{
    err::basic::{ChargeFoundError, ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_module_charge(&mut self, item_id: &ItemId) -> Result<(), RemoveModuleChargeError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.remove_module_charge_internal(item_key)
    }
    pub(in crate::sol) fn remove_module_charge_internal(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), RemoveModuleChargeError> {
        let module = self.uad.items.get(item_key).get_module()?;
        let charge_key = match module.get_charge_item_key() {
            Some(charge_key) => charge_key,
            None => {
                return Err(ChargeFoundError {
                    cont_item_id: module.get_item_id(),
                }
                .into());
            }
        };
        self.remove_charge_internal(charge_key).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveModuleChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
    ChargeNotSet(ChargeFoundError),
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
