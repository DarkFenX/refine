use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_service(&mut self, item_id: &ItemId) -> Result<(), RemoveServiceError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_service_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_service_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let service = item.get_service()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(service.get_fit_key());
        fit.services.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveServiceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotService(#[from] ItemKindMatchError),
}
