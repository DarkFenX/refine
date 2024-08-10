use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        item_info::SolFwEffectInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn get_fw_effect(&self, item_id: &SolItemId) -> Result<SolFwEffectInfo, GetFwEffectError> {
        let fw_effect = self.items.get_item(item_id)?.get_fw_effect()?;
        Ok(SolFwEffectInfo::from(fw_effect))
    }
}

#[derive(Debug)]
pub enum GetFwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFwEffect(ItemKindMatchError),
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
