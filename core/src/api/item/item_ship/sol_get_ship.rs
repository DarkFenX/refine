use crate::{
    api::{Ship, ShipMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_ship(&self, item_id: &ItemId) -> Result<Ship<'_>, GetShipError> {
        let ship_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(ship_uid).dc_ship()?;
        Ok(Ship::new(self, ship_uid))
    }
    pub fn get_ship_mut(&mut self, item_id: &ItemId) -> Result<ShipMut<'_>, GetShipError> {
        let ship_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(ship_uid).dc_ship()?;
        Ok(ShipMut::new(self, ship_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetShipError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
