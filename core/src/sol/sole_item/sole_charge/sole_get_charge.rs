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

#[derive(Debug)]
pub enum GetChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
}
impl std::error::Error for GetChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharge(error)
    }
}
