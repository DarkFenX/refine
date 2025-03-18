use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_service(&mut self, item_id: &SolItemId) -> Result<(), RemoveServiceError> {
        let item = self.uad.items.get_item(item_id)?;
        let service = item.get_service()?;
        self.svc.remove_item(&self.uad, item);
        let fit = self.uad.fits.get_fit_mut(&service.get_fit_id()).unwrap();
        fit.services.remove(item_id);
        self.uad.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveServiceError {
    ItemNotFound(ItemFoundError),
    ItemIsNotService(ItemKindMatchError),
}
impl std::error::Error for RemoveServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotService(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotService(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveServiceError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveServiceError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotService(error)
    }
}
