use crate::{
    api::{SwEffect, SwEffectMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffect<'_>, SwEffectGetError> {
        let sw_effect_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(sw_effect_uid).dc_sw_effect()?;
        Ok(SwEffect::new(self, sw_effect_uid))
    }
    pub fn get_sw_effect_mut(&mut self, item_id: &ItemId) -> Result<SwEffectMut<'_>, SwEffectGetError> {
        let sw_effect_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(sw_effect_uid).dc_sw_effect()?;
        Ok(SwEffectMut::new(self, sw_effect_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SwEffectGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
