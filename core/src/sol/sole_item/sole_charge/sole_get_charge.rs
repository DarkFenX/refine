use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        item_info::SolChargeInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn get_charge(&self, item_id: &SolItemId) -> Result<SolChargeInfo, GetChargeError> {
        let charge = self.items.get_item(item_id)?.get_charge()?;
        Ok(SolChargeInfo::from(charge))
    }
}

#[derive(Debug)]
pub enum GetChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
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
