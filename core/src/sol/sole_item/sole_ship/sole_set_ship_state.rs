use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_ship_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetShipStateError> {
        let ship = self.uad.items.get_mut_by_id(item_id)?.get_ship_mut()?;
        let old_a_state = ship.get_a_state();
        ship.set_ship_state(state);
        let new_a_state = ship.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetShipStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotShip(ItemKindMatchError),
}
impl std::error::Error for SetShipStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotShip(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetShipStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotShip(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetShipStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetShipStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotShip(error)
    }
}
