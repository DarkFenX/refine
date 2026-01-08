use crate::{
    api::{Fighter, FighterMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_fighter(&self, item_id: &ItemId) -> Result<Fighter<'_>, GetFighterError> {
        let fighter_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(fighter_uid).dc_fighter()?;
        Ok(Fighter::new(self, fighter_uid))
    }
    pub fn get_fighter_mut(&mut self, item_id: &ItemId) -> Result<FighterMut<'_>, GetFighterError> {
        let fighter_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(fighter_uid).dc_fighter()?;
        Ok(FighterMut::new(self, fighter_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFighterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFighter(#[from] ItemKindMatchError),
}
