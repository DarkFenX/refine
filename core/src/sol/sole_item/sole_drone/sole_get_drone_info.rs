use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolDroneInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_drone_info(&self, item_id: &SolItemId) -> Result<SolDroneInfo, GetDroneInfoError> {
        let drone = self.items.get_item(item_id)?.get_drone()?;
        Ok(SolDroneInfo::from(drone))
    }
}

#[derive(Debug)]
pub enum GetDroneInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
}
impl From<ItemFoundError> for GetDroneInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetDroneInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
impl std::error::Error for GetDroneInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetDroneInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
        }
    }
}
