use crate::{
    api::{Booster, BoosterMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_booster(&self, item_id: &ItemId) -> Result<Booster<'_>, GetBoosterError> {
        let booster_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(booster_uid).dc_booster()?;
        Ok(Booster::new(self, booster_uid))
    }
    pub fn get_booster_mut(&mut self, item_id: &ItemId) -> Result<BoosterMut<'_>, GetBoosterError> {
        let booster_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(booster_uid).dc_booster()?;
        Ok(BoosterMut::new(self, booster_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetBoosterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
