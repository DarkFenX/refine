use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_stance(&mut self, item_id: &SolItemId) -> Result<(), RemoveStanceError> {
        let item = self.items.get_item(item_id)?;
        let stance = item.get_stance()?;
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        let fit = self.fits.get_fit_mut(&stance.get_fit_id()).unwrap();
        fit.stance = None;
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveStanceError {
    ItemNotFound(ItemFoundError),
    ItemIsNotStance(ItemKindMatchError),
}
impl From<ItemFoundError> for RemoveStanceError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveStanceError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotStance(error)
    }
}
impl std::error::Error for RemoveStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotStance(e) => e.fmt(f),
        }
    }
}
