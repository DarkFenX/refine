use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::DroneInfo},
};

impl SolarSystem {
    pub fn get_drone(&self, item_id: &ItemId) -> Result<DroneInfo, GetDroneError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_drone_internal(item_key)?)
    }
    pub(in crate::sol) fn get_drone_internal(&self, item_key: ItemKey) -> Result<DroneInfo, ItemKindMatchError> {
        let drone = self.uad.items.get(item_key).get_drone()?;
        Ok(DroneInfo::from_drone(&self.uad, drone))
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
