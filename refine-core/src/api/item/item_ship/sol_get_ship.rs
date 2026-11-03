use crate::{
    ItemId, Ship, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn get_ship(&mut self, item_id: &ItemId) -> Result<Ship<'_>, ShipGetError> {
        let ship_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(ship_uid).dc_ship()?;
        Ok(Ship::new(self, ship_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ShipGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
