use crate::{
    api::{SwEffect, SwEffectMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffect<'_>, GetSwEffectError> {
        let sw_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(sw_effect_uid).dc_sw_effect()?;
        Ok(SwEffect::new(self, sw_effect_uid))
    }
    pub fn get_sw_effect_mut(&mut self, item_id: &ItemId) -> Result<SwEffectMut<'_>, GetSwEffectError> {
        let sw_effect_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(sw_effect_uid).dc_sw_effect()?;
        Ok(SwEffectMut::new(self, sw_effect_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
