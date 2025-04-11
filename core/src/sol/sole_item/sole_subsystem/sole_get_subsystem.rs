use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::SubsystemInfo},
};

impl SolarSystem {
    pub fn get_subsystem_info(&self, item_id: &ItemId) -> Result<SubsystemInfo, GetSubsystemInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_subsystem_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_subsystem_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<SubsystemInfo, ItemKindMatchError> {
        let subsystem = self.uad.items.get(item_key).get_subsystem()?;
        Ok(SubsystemInfo::from_subsystem(&self.uad, subsystem))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSubsystemInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
