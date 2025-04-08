use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_booster(&mut self, item_id: &ItemId) -> Result<(), RemoveBoosterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_booster_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_booster_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let booster = item.get_booster()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(booster.get_fit_key());
        fit.boosters.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveBoosterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl std::error::Error for RemoveBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveBoosterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveBoosterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
