use crate::{
    api::{Charge, ChargeMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_charge(&self, item_id: &ItemId) -> Result<Charge<'_>, GetChargeError> {
        let charge_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(charge_key).dc_charge()?;
        Ok(Charge::new(self, charge_key))
    }
    pub fn get_charge_mut(&mut self, item_id: &ItemId) -> Result<ChargeMut<'_>, GetChargeError> {
        let charge_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(charge_key).dc_charge()?;
        Ok(ChargeMut::new(self, charge_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetChargeError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharge(#[from] ItemKindMatchError),
}
