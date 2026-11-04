use crate::{Fleet, FleetId, Sol, err::basic::FleetFoundError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fleet(&mut self, fleet_id: &FleetId) -> Result<Fleet<'_>, FleetGetError> {
        let core = self.inner_mut();
        let fleet_uid = core.u_data.fleets.int_id_by_ext_id_err(fleet_id)?;
        Ok(Fleet::new(core, fleet_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetGetError {
    #[error(transparent)]
    FleetNotFound(#[from] FleetFoundError),
}
