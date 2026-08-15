use crate::{ChangeFleetCmd, Fleet, FleetInfo, FleetInfoArgs, err::FleetChangeFleetError};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeFleetCmd) -> Result<(), ChangeFleetError> {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                cmd.execute(&mut core_fleet)
            })
            .await?;
        Ok(())
    }
    #[tracing::instrument(name = "flt-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        exec_cmd: ChangeFleetCmd,
        info_args: FleetInfoArgs,
    ) -> Result<FleetInfo, ChangeFleetError> {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                exec_cmd.execute(&mut core_fleet)?;
                let item_info = FleetInfo::from_core(&mut core_fleet, info_args);
                Ok(item_info)
            })
            .await
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ChangeFleetError(#[from] pub FleetChangeFleetError);
