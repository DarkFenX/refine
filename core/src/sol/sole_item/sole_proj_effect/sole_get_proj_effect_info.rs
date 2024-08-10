use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolProjEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_proj_effect_info(&self, item_id: &SolItemId) -> Result<SolProjEffectInfo, GetProjEffectInfoError> {
        let proj_effect = self.items.get_item(item_id)?.get_proj_effect().unwrap();
        Ok(SolProjEffectInfo::from(proj_effect))
    }
}

#[derive(Debug)]
pub enum GetProjEffectInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotProjEffect(ItemKindMatchError),
}
impl From<ItemFoundError> for GetProjEffectInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetProjEffectInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotProjEffect(error)
    }
}
impl std::error::Error for GetProjEffectInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotProjEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetProjEffectInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotProjEffect(e) => e.fmt(f),
        }
    }
}
