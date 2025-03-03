use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolarSystem, uad::item::SolShipKind},
};

impl SolarSystem {
    pub fn remove_ship(&mut self, item_id: &SolItemId) -> Result<(), RemoveShipError> {
        // Just check if everything is correct
        let item = self.uad.items.get_item(item_id)?;
        let ship = item.get_ship()?;
        let fit_id = ship.get_fit_id();
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove ship from services
        self.remove_item_id_from_svc(item_id);
        // Remove ship from user data
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.ship = None;
        fit.kind = SolShipKind::Unknown;
        self.uad.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveShipError {
    ItemNotFound(ItemFoundError),
    ItemIsNotShip(ItemKindMatchError),
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
