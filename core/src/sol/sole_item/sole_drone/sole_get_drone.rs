use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{info::SolDroneInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_drone(&self, item_id: &SolItemId) -> Result<SolDroneInfo, GetDroneError> {
        let drone = self.uad.items.get_item(item_id)?.get_drone()?;
        Ok(self.make_drone_info(drone))
    }
}

#[derive(Debug)]
pub enum GetDroneError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
}
impl std::error::Error for GetDroneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetDroneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetDroneError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetDroneError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
