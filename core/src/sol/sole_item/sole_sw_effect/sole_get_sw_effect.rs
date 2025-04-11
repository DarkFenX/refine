use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::SwEffectInfo},
};

impl SolarSystem {
    pub fn get_sw_effect_info(&self, item_id: &ItemId) -> Result<SwEffectInfo, GetSwEffectInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_sw_effect_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_sw_effect_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<SwEffectInfo, ItemKindMatchError> {
        let sw_effect = self.uad.items.get(item_key).get_sw_effect()?;
        Ok(SwEffectInfo::from_sw_effect(sw_effect))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSwEffectInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
