use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_sw_effect(&mut self, item_id: &ItemId) -> Result<(), RemoveSwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_sw_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_sw_effect_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        // Just to check item kind
        item.get_sw_effect()?;
        self.svc.remove_item(&self.uad, item_key, item);
        self.uad.sw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveSwEffectError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSwEffect(#[from] ItemKindMatchError),
}
