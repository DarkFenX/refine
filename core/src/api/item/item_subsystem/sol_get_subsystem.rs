use crate::{
    api::{Subsystem, SubsystemMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ItemId) -> Result<Subsystem<'_>, GetSubsystemError> {
        let subsystem_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(Subsystem::new(self, subsystem_uid))
    }
    pub fn get_subsystem_mut(&mut self, item_id: &ItemId) -> Result<SubsystemMut<'_>, GetSubsystemError> {
        let subsystem_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(SubsystemMut::new(self, subsystem_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSubsystemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
