use crate::{
    api::{ProjEffect, ProjEffectMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_proj_effect(&self, item_id: &ItemId) -> Result<ProjEffect<'_>, GetProjEffectError> {
        let proj_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(proj_effect_uid).dc_proj_effect()?;
        Ok(ProjEffect::new(self, proj_effect_uid))
    }
    pub fn get_proj_effect_mut(&mut self, item_id: &ItemId) -> Result<ProjEffectMut<'_>, GetProjEffectError> {
        let proj_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(proj_effect_uid).dc_proj_effect()?;
        Ok(ProjEffectMut::new(self, proj_effect_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetProjEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotProjEffect(#[from] ItemKindMatchError),
}
