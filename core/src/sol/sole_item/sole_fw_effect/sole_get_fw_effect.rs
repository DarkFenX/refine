use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::FwEffectInfo},
};

impl SolarSystem {
    pub fn get_fw_effect(&self, item_id: &ItemId) -> Result<FwEffectInfo, GetFwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_fw_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn get_fw_effect_internal(&self, item_key: ItemKey) -> Result<FwEffectInfo, ItemKindMatchError> {
        let fw_effect = self.uad.items.get(item_key).get_fw_effect()?;
        Ok(FwEffectInfo::from_fw_effect(&self.uad, fw_effect))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotFwEffect(#[from] ItemKindMatchError),
}
