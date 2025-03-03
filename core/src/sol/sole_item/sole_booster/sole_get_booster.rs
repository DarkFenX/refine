use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolarSystem, info::SolBoosterInfo},
};

impl SolarSystem {
    pub fn get_booster(&self, item_id: &SolItemId) -> Result<SolBoosterInfo, GetBoosterError> {
        let booster = self.uad.items.get_item(item_id)?.get_booster()?;
        Ok(self.make_booster_info(booster))
    }
}

#[derive(Debug)]
pub enum GetBoosterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl std::error::Error for GetBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetBoosterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetBoosterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
