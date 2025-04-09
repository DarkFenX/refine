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

#[derive(thiserror::Error, Debug)]
pub enum RemoveModuleChargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
    #[error("{0}")]
    ChargeNotSet(#[from] ChargeFoundError),
}
