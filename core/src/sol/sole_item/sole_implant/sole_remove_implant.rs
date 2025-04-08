use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_implant(&mut self, item_id: &ItemId) -> Result<(), RemoveImplantError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_implant_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_implant_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let implant = item.get_implant()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(implant.get_fit_key());
        fit.implants.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveImplantError {
    ItemNotFound(ItemFoundError),
    ItemIsNotImplant(ItemKindMatchError),
}
impl std::error::Error for RemoveImplantError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotImplant(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveImplantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotImplant(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveImplantError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveImplantError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotImplant(error)
    }
}
