use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        ItemId, SolarSystem,
        api::{ProjEffect, ProjEffectMut},
    },
};

impl SolarSystem {
    pub fn get_proj_effect(&self, item_id: &ItemId) -> Result<ProjEffect<'_>, GetProjEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_proj_effect()?;
        Ok(ProjEffect::new(self, item_key))
    }
    pub fn get_proj_effect_mut(&mut self, item_id: &ItemId) -> Result<ProjEffectMut<'_>, GetProjEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_proj_effect()?;
        Ok(ProjEffectMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotProjEffect(#[from] ItemKindMatchError),
}
