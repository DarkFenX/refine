use crate::{
    api::{Subsystem, SubsystemMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<Subsystem<'_>, GetSubsystemError> {
        let subsystem_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(Subsystem::new(self, subsystem_uid))
    }
    pub fn get_subsystem_mut(&mut self, item_id: &ItemId) -> Result<SubsystemMut<'_>, GetSubsystemError> {
        let subsystem_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(SubsystemMut::new(self, subsystem_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubsystemError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
