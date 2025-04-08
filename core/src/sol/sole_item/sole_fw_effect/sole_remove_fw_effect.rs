use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fw_effect(&mut self, item_id: &ItemId) -> Result<(), RemoveFwEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_fw_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_fw_effect_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let fw_effect = item.get_fw_effect()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(fw_effect.get_fit_key());
        fit.fw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFwEffect(ItemKindMatchError),
}
impl std::error::Error for RemoveFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveFwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveFwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFwEffect(error)
    }
}
