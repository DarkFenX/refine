use crate::{
    api::{Charge, ChargeMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_charge(&self, item_id: &ItemId) -> Result<Charge<'_>, ChargeGetError> {
        let charge_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(charge_uid).dc_charge()?;
        Ok(Charge::new(self, charge_uid))
    }
    pub fn get_charge_mut(&mut self, item_id: &ItemId) -> Result<ChargeMut<'_>, ChargeGetError> {
        let charge_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(charge_uid).dc_charge()?;
        Ok(ChargeMut::new(self, charge_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChargeGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotCharge(#[from] ItemKindMatchError),
}
