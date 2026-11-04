use crate::{
    ItemId, Ship, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_ship(&mut self, item_id: &ItemId) -> Result<Ship<'_>, ShipGetError> {
        let core = self.inner_mut();
        let ship_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(ship_uid).dc_ship()?;
        Ok(Ship::new(core, ship_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ShipGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
