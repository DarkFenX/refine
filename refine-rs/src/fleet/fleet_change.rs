use crate::{
    cmd::{BasicChangeFleetError, ChangeFleetCmd},
    fleet::Fleet,
};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeFleetCmd) -> Result<(), ChangeFleetError> {
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol, &fleet_id))
            .await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFleetError {
    #[error("failed to add fit: {0}")]
    FitAddFailed(#[source] rc::err::FleetAddFitError),
    #[error("failed to remove fit: {0}")]
    FitRemoveFailed(#[source] rc::err::FleetRemoveFitError),
}
impl From<BasicChangeFleetError> for ChangeFleetError {
    fn from(error: BasicChangeFleetError) -> Self {
        match error {
            // Holding mutex on sol - nothing can remove the core fleet without consuming the
            // high-level Fleet
            BasicChangeFleetError::FleetGetFailed(_) => unreachable!(),
            BasicChangeFleetError::FitAddFailed(core_error) => Self::FitAddFailed(core_error),
            BasicChangeFleetError::FitRemoveFailed(core_error) => Self::FitRemoveFailed(core_error),
        }
    }
}
