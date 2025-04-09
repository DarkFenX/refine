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

#[derive(thiserror::Error, Debug)]
pub enum GetFighterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
