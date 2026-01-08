use crate::{
    api::{FwEffect, FwEffectMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_fw_effect(&self, item_id: &ItemId) -> Result<FwEffect<'_>, GetFwEffectError> {
        let fw_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffect::new(self, fw_effect_uid))
    }
    pub fn get_fw_effect_mut(&mut self, item_id: &ItemId) -> Result<FwEffectMut<'_>, GetFwEffectError> {
        let fw_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffectMut::new(self, fw_effect_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
