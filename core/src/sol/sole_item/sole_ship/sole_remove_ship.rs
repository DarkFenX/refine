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

#[derive(thiserror::Error, Debug)]
pub enum RemoveShipError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
