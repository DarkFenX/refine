use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ChargeInfo},
};

impl SolarSystem {
    pub fn get_charge(&self, item_id: &ItemId) -> Result<ChargeInfo, GetChargeError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_charge_internal(item_key)?)
    }
    pub(in crate::sol) fn get_charge_internal(&self, item_key: ItemKey) -> Result<ChargeInfo, ItemKindMatchError> {
        let charge = self.uad.items.get(item_key).get_charge()?;
        Ok(ChargeInfo::from_charge(&self.uad, charge))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetChargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharge(#[from] ItemKindMatchError),
}
