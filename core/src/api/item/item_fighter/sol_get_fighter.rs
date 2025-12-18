use crate::{
    api::{Fighter, FighterMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_fighter(&self, item_id: &ItemId) -> Result<Fighter<'_>, GetFighterError> {
        let fighter_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(fighter_key).dc_fighter()?;
        Ok(Fighter::new(self, fighter_key))
    }
    pub fn get_fighter_mut(&mut self, item_id: &ItemId) -> Result<FighterMut<'_>, GetFighterError> {
        let fighter_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(fighter_key).dc_fighter()?;
        Ok(FighterMut::new(self, fighter_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFighterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
