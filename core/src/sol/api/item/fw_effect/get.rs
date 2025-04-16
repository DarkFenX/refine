use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        ItemId, SolarSystem,
        api::{FwEffect, FwEffectMut},
    },
};

impl SolarSystem {
    pub fn get_fw_effect(&self, item_id: &ItemId) -> Result<FwEffect, GetFwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_fw_effect()?;
        Ok(FwEffect::new(self, item_key))
    }
    pub fn get_fw_effect_mut(&mut self, item_id: &ItemId) -> Result<FwEffectMut, GetFwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_fw_effect()?;
        Ok(FwEffectMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
