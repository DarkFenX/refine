use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolBoosterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_booster_info(&self, item_id: &SolItemId) -> Result<SolBoosterInfo, GetBoosterInfoError> {
        let booster = self.items.get_item(item_id)?.get_booster()?;
        Ok(self.make_booster_info(booster))
    }
}

#[derive(Debug)]
pub enum GetBoosterInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl From<ItemFoundError> for GetBoosterInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetBoosterInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
impl std::error::Error for GetBoosterInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetBoosterInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
