use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fw_effect(&mut self, item_id: &ItemId) -> Result<(), RemoveFwEffectError> {
        let item = self.uad.items.get_by_id(item_id)?;
        let fw_effect = item.get_fw_effect()?;
        self.svc.remove_item(&self.uad, item);
        let fit = self.uad.fits.get_fit_mut(&fw_effect.get_fit_id()).unwrap();
        fit.fw_effects.remove(item_id);
        self.uad.items.remove_by_id(item_id);
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
