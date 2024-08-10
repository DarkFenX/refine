use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolChargeInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_charge_info(&self, item_id: &SolItemId) -> Result<SolChargeInfo, GetChargeInfoError> {
        let charge = self.items.get_item(item_id)?.get_charge()?;
        Ok(SolChargeInfo::from(charge))
    }
}

#[derive(Debug)]
pub enum GetChargeInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
}
impl From<ItemFoundError> for GetChargeInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetChargeInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharge(error)
    }
}
impl std::error::Error for GetChargeInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetChargeInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharge(e) => e.fmt(f),
        }
    }
}
