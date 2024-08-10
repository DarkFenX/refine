use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolAutoChargeInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_autocharge_info(&self, item_id: &SolItemId) -> Result<SolAutoChargeInfo, GetAutochargeInfoError> {
        let autocharge = self.items.get_item(item_id)?.get_autocharge()?;
        Ok(SolAutoChargeInfo::from(autocharge))
    }
}

#[derive(Debug)]
pub enum GetAutochargeInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotAutocharge(ItemKindMatchError),
}
impl From<ItemFoundError> for GetAutochargeInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetAutochargeInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotAutocharge(error)
    }
}
impl std::error::Error for GetAutochargeInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotAutocharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetAutochargeInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotAutocharge(e) => e.fmt(f),
        }
    }
}
