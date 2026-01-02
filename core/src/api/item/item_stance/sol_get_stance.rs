use crate::{
    api::{Stance, StanceMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_stance(&self, item_id: &ItemId) -> Result<Stance<'_>, GetStanceError> {
        let stance_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(stance_key).dc_stance()?;
        Ok(Stance::new(self, stance_key))
    }
    pub fn get_stance_mut(&mut self, item_id: &ItemId) -> Result<StanceMut<'_>, GetStanceError> {
        let stance_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(stance_key).dc_stance()?;
        Ok(StanceMut::new(self, stance_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetStanceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotStance(#[from] ItemKindMatchError),
}
