use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{SwEffect, SwEffectMut},
    },
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffect<'_>, GetSwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_sw_effect()?;
        Ok(SwEffect::new(self, item_key))
    }
    pub fn get_sw_effect_mut(&mut self, item_id: &ItemId) -> Result<SwEffectMut<'_>, GetSwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_sw_effect()?;
        Ok(SwEffectMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
