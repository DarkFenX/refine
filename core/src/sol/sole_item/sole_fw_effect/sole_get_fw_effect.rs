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

#[derive(Debug)]
pub enum GetFwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFwEffect(ItemKindMatchError),
}
impl std::error::Error for GetFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetFwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetFwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFwEffect(error)
    }
}
