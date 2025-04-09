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

#[derive(thiserror::Error, Debug)]
pub enum RemoveImplantError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
