use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::SwEffectInfo},
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffectInfo, GetSwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_sw_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn get_sw_effect_internal(&self, item_key: ItemKey) -> Result<SwEffectInfo, ItemKindMatchError> {
        let sw_effect = self.uad.items.get(item_key).get_sw_effect()?;
        Ok(SwEffectInfo::from_sw_effect(sw_effect))
    }
}

#[derive(Debug)]
pub enum GetSwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl std::error::Error for GetSwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetSwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
