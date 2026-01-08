use crate::{
    api::{Stance, StanceMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_stance(&self, item_id: &ItemId) -> Result<Stance<'_>, GetStanceError> {
        let stance_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(stance_uid).dc_stance()?;
        Ok(Stance::new(self, stance_uid))
    }
    pub fn get_stance_mut(&mut self, item_id: &ItemId) -> Result<StanceMut<'_>, GetStanceError> {
        let stance_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(stance_uid).dc_stance()?;
        Ok(StanceMut::new(self, stance_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetStanceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotStance(#[from] ItemKindMatchError),
}
