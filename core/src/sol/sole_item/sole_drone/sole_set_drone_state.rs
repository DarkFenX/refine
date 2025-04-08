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

#[derive(Debug)]
pub enum SetDroneStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
}
impl std::error::Error for SetDroneStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetDroneStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetDroneStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetDroneStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
