use crate::{ChangeFleetCmd, Fleet, FleetInfo, FleetInfoCmd, err::FleetChangeFleetError};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: ChangeFleetCmd) -> Result<(), ChangeFleetError> {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                ctl_cmd.execute(&mut core_fleet)
            })
            .await?;
        Ok(())
    }
    #[tracing::instrument(name = "flt-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmd: ChangeFleetCmd,
        info_cmd: FleetInfoCmd,
    ) -> Result<FleetInfo, ChangeFleetError> {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                ctl_cmd.execute(&mut core_fleet)?;
                let fleet_info = info_cmd.execute(&mut core_fleet);
                Ok(fleet_info)
            })
            .await
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ChangeFleetError(#[from] pub FleetChangeFleetError);
