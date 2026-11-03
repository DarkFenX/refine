use crate::{
    api::Booster,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_booster(&mut self, item_id: &ItemId) -> Result<Booster<'_>, BoosterGetError> {
        let booster_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(booster_uid).dc_booster()?;
        Ok(Booster::new(self, booster_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoosterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
