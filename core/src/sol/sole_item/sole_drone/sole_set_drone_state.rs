use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::MinionState},
};

impl SolarSystem {
    pub fn set_drone_state(&mut self, item_id: &ItemId, state: MinionState) -> Result<(), SetDroneStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_drone_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_drone_state_internal(
        &mut self,
        item_key: ItemKey,
        state: MinionState,
    ) -> Result<(), ItemKindMatchError> {
        let drone = self.uad.items.get_mut(item_key).get_drone_mut()?;
        let old_a_state = drone.get_a_state();
        drone.set_drone_state(state);
        let new_a_state = drone.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetDroneStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
