use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Fighter, FighterMut},
    },
};

impl SolarSystem {
    pub fn get_fighter(&self, item_id: &ItemId) -> Result<Fighter<'_>, GetFighterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_fighter()?;
        Ok(Fighter::new(self, item_key))
    }
    pub fn get_fighter_mut(&mut self, item_id: &ItemId) -> Result<FighterMut<'_>, GetFighterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_fighter()?;
        Ok(FighterMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFighterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
