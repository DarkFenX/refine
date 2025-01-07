use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{info::SolProjEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_proj_effect(&self, item_id: &SolItemId) -> Result<SolProjEffectInfo, GetProjEffectError> {
        let proj_effect = self.uad.items.get_item(item_id)?.get_proj_effect().unwrap();
        Ok(SolProjEffectInfo::from(proj_effect))
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
