use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Ship, ShipMut},
    },
};

impl SolarSystem {
    pub fn get_ship(&self, item_id: &ItemId) -> Result<Ship<'_>, GetShipError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_ship()?;
        Ok(Ship::new(self, item_key))
    }
    pub fn get_ship_mut(&mut self, item_id: &ItemId) -> Result<ShipMut<'_>, GetShipError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_ship()?;
        Ok(ShipMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetShipError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
