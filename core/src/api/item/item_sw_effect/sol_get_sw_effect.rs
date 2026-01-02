use crate::{
    api::{SwEffect, SwEffectMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffect<'_>, GetSwEffectError> {
        let sw_effect_key = self.u_data.items.iid_by_eid_err(item_id)?;
        self.u_data.items.get(sw_effect_key).dc_sw_effect()?;
        Ok(SwEffect::new(self, sw_effect_key))
    }
    pub fn get_sw_effect_mut(&mut self, item_id: &ItemId) -> Result<SwEffectMut<'_>, GetSwEffectError> {
        let sw_effect_key = self.u_data.items.iid_by_eid_err(item_id)?;
        self.u_data.items.get(sw_effect_key).dc_sw_effect()?;
        Ok(SwEffectMut::new(self, sw_effect_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
