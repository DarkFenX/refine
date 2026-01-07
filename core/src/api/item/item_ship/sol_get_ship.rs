use crate::{
    api::{Ship, ShipMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_ship(&self, item_id: &ItemId) -> Result<Ship<'_>, GetShipError> {
        let ship_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(ship_key).dc_ship()?;
        Ok(Ship::new(self, ship_key))
    }
    pub fn get_ship_mut(&mut self, item_id: &ItemId) -> Result<ShipMut<'_>, GetShipError> {
        let ship_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(ship_key).dc_ship()?;
        Ok(ShipMut::new(self, ship_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetShipError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
