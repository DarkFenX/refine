use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ProjEffectInfo},
};

impl SolarSystem {
    pub fn get_proj_effect(&self, item_id: &ItemId) -> Result<ProjEffectInfo, GetProjEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_proj_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn get_proj_effect_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<ProjEffectInfo, ItemKindMatchError> {
        let proj_effect = self.uad.items.get(item_key).get_proj_effect().unwrap();
        Ok(ProjEffectInfo::from_proj_effect(&self.uad, proj_effect))
    }
}

#[derive(Debug)]
pub enum GetProjEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotProjEffect(ItemKindMatchError),
}
impl std::error::Error for GetProjEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotProjEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetProjEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotProjEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetProjEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetProjEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotProjEffect(error)
    }
}
