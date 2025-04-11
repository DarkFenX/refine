use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ProjEffectInfo},
};

impl SolarSystem {
    pub fn get_proj_effect_info(&self, item_id: &ItemId) -> Result<ProjEffectInfo, GetProjEffectInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_proj_effect_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_proj_effect_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<ProjEffectInfo, ItemKindMatchError> {
        let proj_effect = self.uad.items.get(item_key).get_proj_effect().unwrap();
        Ok(ProjEffectInfo::from_proj_effect(&self.uad, proj_effect))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjEffectInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotProjEffect(#[from] ItemKindMatchError),
}
