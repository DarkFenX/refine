use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolFighterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fighter_info(&self, item_id: &SolItemId) -> Result<SolFighterInfo, GetFighterInfoError> {
        let fighter = self.items.get_item(item_id)?.get_fighter()?;
        Ok(self.make_fighter_info(fighter))
    }
}

#[derive(Debug)]
pub enum GetFighterInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFighter(ItemKindMatchError),
}
impl From<ItemFoundError> for GetFighterInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetFighterInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFighter(error)
    }
}
impl std::error::Error for GetFighterInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFighter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFighterInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFighter(e) => e.fmt(f),
        }
    }
}
