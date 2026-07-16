use crate::{Fleet, SolarSystem};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "flt-get", level = "trace", skip_all)]
    pub async fn get_fleet(&'s mut self, fleet_id: rc::FleetId) -> Result<Fleet<'r, 's>, GetFleetError> {
        let fleet_id = self.exec_inplace(move |core_sol| {
            core_sol
                .get_fleet(&fleet_id)
                .map(|core_fleet| core_fleet.get_fleet_id())
        })?;
        let fleet = Fleet::new(self, fleet_id);
        Ok(fleet)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct GetFleetError(#[from] pub rc::err::GetFleetError);
