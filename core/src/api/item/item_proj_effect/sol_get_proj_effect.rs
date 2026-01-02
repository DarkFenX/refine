use crate::{
    api::{ProjEffect, ProjEffectMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_proj_effect(&self, item_id: &ItemId) -> Result<ProjEffect<'_>, GetProjEffectError> {
        let proj_effect_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(proj_effect_key).dc_proj_effect()?;
        Ok(ProjEffect::new(self, proj_effect_key))
    }
    pub fn get_proj_effect_mut(&mut self, item_id: &ItemId) -> Result<ProjEffectMut<'_>, GetProjEffectError> {
        let proj_effect_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(proj_effect_key).dc_proj_effect()?;
        Ok(ProjEffectMut::new(self, proj_effect_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotProjEffect(#[from] ItemKindMatchError),
}
