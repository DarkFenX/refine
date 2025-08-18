use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Subsystem, SubsystemMut},
    },
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<Subsystem<'_>, GetSubsystemError> {
        let subsystem_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(subsystem_key).get_subsystem()?;
        Ok(Subsystem::new(self, subsystem_key))
    }
    pub fn get_subsystem_mut(&mut self, item_id: &ItemId) -> Result<SubsystemMut<'_>, GetSubsystemError> {
        let subsystem_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(subsystem_key).get_subsystem()?;
        Ok(SubsystemMut::new(self, subsystem_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSubsystemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
