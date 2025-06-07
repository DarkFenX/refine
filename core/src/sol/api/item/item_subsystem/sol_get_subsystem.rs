use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        ItemId, SolarSystem,
        api::{Subsystem, SubsystemMut},
    },
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<Subsystem<'_>, GetSubsystemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_subsystem()?;
        Ok(Subsystem::new(self, item_key))
    }
    pub fn get_subsystem_mut(&mut self, item_id: &ItemId) -> Result<SubsystemMut<'_>, GetSubsystemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_subsystem()?;
        Ok(SubsystemMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSubsystemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
