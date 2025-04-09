use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_ship_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetShipStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_ship_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_ship_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let ship = self.uad.items.get_mut(item_key).get_ship_mut()?;
        let old_a_state = ship.get_a_state();
        ship.set_ship_state(state);
        let new_a_state = ship.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetShipStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotShip(#[from] ItemKindMatchError),
}
