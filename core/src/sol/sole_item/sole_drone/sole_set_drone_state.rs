use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{uad::item::SolMinionState, SolarSystem},
};

impl SolarSystem {
    pub fn set_drone_state(&mut self, item_id: &SolItemId, state: SolMinionState) -> Result<(), SetDroneStateError> {
        let drone = self.uad.items.get_item_mut(item_id)?.get_drone_mut()?;
        let old_state = drone.get_state();
        drone.set_drone_state(state);
        self.change_item_id_state_in_svc(item_id, old_state, state.into());
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
