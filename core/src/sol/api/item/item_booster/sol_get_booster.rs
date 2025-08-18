use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Booster, BoosterMut},
    },
};

impl SolarSystem {
    pub fn get_booster(&self, item_id: &ItemId) -> Result<Booster<'_>, GetBoosterError> {
        let booster_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(booster_key).get_booster()?;
        Ok(Booster::new(self, booster_key))
    }
    pub fn get_booster_mut(&mut self, item_id: &ItemId) -> Result<BoosterMut<'_>, GetBoosterError> {
        let booster_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(booster_key).get_booster()?;
        Ok(BoosterMut::new(self, booster_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetBoosterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
