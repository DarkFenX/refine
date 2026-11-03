use crate::{
    ItemId, SolarSystem, Subsystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

impl SolarSystem {
    pub fn get_subsystem(&mut self, item_id: &ItemId) -> Result<Subsystem<'_>, SubsystemGetError> {
        let subsystem_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(Subsystem::new(self, subsystem_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SubsystemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
