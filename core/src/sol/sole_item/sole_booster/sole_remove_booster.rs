use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_booster(&mut self, item_id: &SolItemId) -> Result<(), RemoveBoosterError> {
        let item = self.items.get_item(item_id)?;
        let booster = item.get_booster()?;
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        let fit = self.fits.get_fit_mut(&booster.get_fit_id()).unwrap();
        fit.boosters.remove(item_id);
        self.items.remove_item(item_id);
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
