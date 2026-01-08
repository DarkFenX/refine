use crate::{
    api::{Charge, ChargeMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_charge(&self, item_id: &ItemId) -> Result<Charge<'_>, GetChargeError> {
        let charge_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(charge_uid).dc_charge()?;
        Ok(Charge::new(self, charge_uid))
    }
    pub fn get_charge_mut(&mut self, item_id: &ItemId) -> Result<ChargeMut<'_>, GetChargeError> {
        let charge_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(charge_uid).dc_charge()?;
        Ok(ChargeMut::new(self, charge_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetChargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharge(#[from] ItemKindMatchError),
}
