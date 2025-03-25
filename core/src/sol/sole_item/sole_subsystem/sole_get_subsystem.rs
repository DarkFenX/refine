use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, info::SubsystemInfo},
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<SubsystemInfo, GetSubsystemError> {
        let subsystem = self.uad.items.get_item(item_id)?.get_subsystem()?;
        Ok(SubsystemInfo::from(subsystem))
    }
}

#[derive(Debug)]
pub enum GetSubsystemError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSubsystem(ItemKindMatchError),
}
impl std::error::Error for GetSubsystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSubsystem(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSubsystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSubsystem(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetSubsystemError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSubsystemError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSubsystem(error)
    }
}
