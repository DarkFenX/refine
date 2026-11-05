use crate::{
    ItemId, Sol, Subsystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_subsystem(&mut self, item_id: &ItemId) -> Result<Subsystem<'_>, SubsystemGetError> {
        let sol = self.inner_mut();
        let subsystem_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(subsystem_uid).dc_subsystem()?;
        Ok(Subsystem::new(sol, subsystem_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SubsystemGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
