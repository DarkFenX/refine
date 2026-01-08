use crate::{
    api::{Autocharge, AutochargeMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_autocharge(&self, item_id: &ItemId) -> Result<Autocharge<'_>, GetAutochargeError> {
        let autocharge_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(autocharge_uid).dc_autocharge()?;
        Ok(Autocharge::new(self, autocharge_uid))
    }
    pub fn get_autocharge_mut(&mut self, item_id: &ItemId) -> Result<AutochargeMut<'_>, GetAutochargeError> {
        let autocharge_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(autocharge_uid).dc_autocharge()?;
        Ok(AutochargeMut::new(self, autocharge_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetAutochargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
