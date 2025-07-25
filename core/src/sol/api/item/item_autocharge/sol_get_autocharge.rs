use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Autocharge, AutochargeMut},
    },
};

impl SolarSystem {
    pub fn get_autocharge(&self, item_id: &ItemId) -> Result<Autocharge<'_>, GetAutochargeError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_autocharge()?;
        Ok(Autocharge::new(self, item_key))
    }
    pub fn get_autocharge_mut(&mut self, item_id: &ItemId) -> Result<AutochargeMut<'_>, GetAutochargeError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_autocharge()?;
        Ok(AutochargeMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetAutochargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
