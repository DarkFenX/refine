use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::SubsystemInfo},
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<SubsystemInfo, GetSubsystemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_subsystem_internal(item_key)?)
    }
    pub(in crate::sol) fn get_subsystem_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<SubsystemInfo, ItemKindMatchError> {
        let subsystem = self.uad.items.get(item_key).get_subsystem()?;
        Ok(SubsystemInfo::from_subsystem(&self.uad, subsystem))
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
