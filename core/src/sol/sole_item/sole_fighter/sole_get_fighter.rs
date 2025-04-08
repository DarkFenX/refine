use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::FighterInfo},
};

impl SolarSystem {
    pub fn get_fighter(&self, item_id: &ItemId) -> Result<FighterInfo, GetFighterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_fighter_internal(item_key)?)
    }
    pub(in crate::sol) fn get_fighter_internal(&self, item_key: ItemKey) -> Result<FighterInfo, ItemKindMatchError> {
        let fighter = self.uad.items.get(item_key).get_fighter()?;
        Ok(FighterInfo::from_fighter(&self.uad, fighter))
    }
}

#[derive(Debug)]
pub enum GetFighterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
}
impl std::error::Error for GetFighterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFighterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetFighterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetFighterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
