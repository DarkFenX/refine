use crate::{
    api::{Booster, BoosterMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_booster(&self, item_id: &ItemId) -> Result<Booster<'_>, GetBoosterError> {
        let booster_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(booster_key).dc_booster()?;
        Ok(Booster::new(self, booster_key))
    }
    pub fn get_booster_mut(&mut self, item_id: &ItemId) -> Result<BoosterMut<'_>, GetBoosterError> {
        let booster_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(booster_key).dc_booster()?;
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
