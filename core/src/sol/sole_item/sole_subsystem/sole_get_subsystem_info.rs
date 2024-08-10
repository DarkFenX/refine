use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolSubsystemInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_subsystem_info(&self, item_id: &SolItemId) -> Result<SolSubsystemInfo, GetSubsystemInfoError> {
        let subsystem = self.items.get_item(item_id)?.get_subsystem()?;
        Ok(SolSubsystemInfo::from(subsystem))
    }
}

#[derive(Debug)]
pub enum GetSubsystemInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSubsystem(ItemKindMatchError),
}
impl From<ItemFoundError> for GetSubsystemInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSubsystemInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSubsystem(error)
    }
}
impl std::error::Error for GetSubsystemInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSubsystem(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSubsystemInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSubsystem(e) => e.fmt(f),
        }
    }
}
