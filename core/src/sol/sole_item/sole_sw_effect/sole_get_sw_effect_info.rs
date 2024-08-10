use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolSwEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_sw_effect_info(&self, item_id: &SolItemId) -> Result<SolSwEffectInfo, GetSwEffectInfoError> {
        let sw_effect = self.items.get_item(item_id)?.get_sw_effect()?;
        Ok(SolSwEffectInfo::from(sw_effect))
    }
}

#[derive(Debug)]
pub enum GetSwEffectInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl From<ItemFoundError> for GetSwEffectInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSwEffectInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
impl std::error::Error for GetSwEffectInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSwEffectInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
