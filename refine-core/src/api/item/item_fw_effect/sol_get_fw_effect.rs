use crate::{
    api::{FwEffect, FwEffectMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_fw_effect(&self, item_id: &ItemId) -> Result<FwEffect<'_>, GetFwEffectError> {
        let fw_effect_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffect::new(self, fw_effect_uid))
    }
    pub fn get_fw_effect_mut(&mut self, item_id: &ItemId) -> Result<FwEffectMut<'_>, GetFwEffectError> {
        let fw_effect_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(fw_effect_uid).dc_fw_effect()?;
        Ok(FwEffectMut::new(self, fw_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetFwEffectError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
