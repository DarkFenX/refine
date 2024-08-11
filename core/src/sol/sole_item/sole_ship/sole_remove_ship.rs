use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{item::SolShipKind, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_ship(&mut self, item_id: &SolItemId) -> Result<(), RemoveShipError> {
        // Just check if everything is correct
        let item = self.items.get_item(item_id)?;
        item.get_ship()?;
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove ship from services
        let item = self.items.get_item(item_id).unwrap();
        let ship = item.get_ship().unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Remove ship from skeleton
        let fit = self.fits.get_fit_mut(&ship.get_fit_id()).unwrap();
        fit.ship = None;
        fit.kind = SolShipKind::default();
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveShipError {
    ItemNotFound(ItemFoundError),
    ItemIsNotShip(ItemKindMatchError),
}
impl From<ItemFoundError> for RemoveShipError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveShipError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotShip(error)
    }
}
impl std::error::Error for RemoveShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotShip(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotShip(e) => e.fmt(f),
        }
    }
}
