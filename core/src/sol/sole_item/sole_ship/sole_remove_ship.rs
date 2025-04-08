use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ShipKind},
};

impl SolarSystem {
    pub fn remove_ship(&mut self, item_id: &ItemId) -> Result<(), RemoveShipError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_ship_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_ship_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        // Just check if everything is correct
        let item = self.uad.items.get(item_key);
        let ship = item.get_ship()?;
        let fit_key = ship.get_fit_key();
        // Remove incoming projections
        self.remove_incoming_projections(item_key);
        // Remove ship from services
        self.remove_item_key_from_svc(item_key);
        // Remove ship from user data
        let fit = self.uad.fits.get_mut(fit_key);
        fit.ship = None;
        fit.kind = ShipKind::Unknown;
        self.uad.items.remove(item_key);
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
