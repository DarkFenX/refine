use crate::{
    cmd::{ChangeFleetCmd, FleetChangeFleetError},
    fleet::Fleet,
    info::{FleetInfo, FleetInfoMode},
};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeFleetCmd) -> Result<(), ChangeFleetError> {
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
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        cmd: ChangeFleetCmd,
        fleet_mode: FleetInfoMode,
    ) -> Result<FleetInfo, ChangeFleetError> {
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                cmd.execute(&mut core_fleet)?;
                let item_info = FleetInfo::from_core(&mut core_fleet, fleet_mode);
                Ok(item_info)
            })
            .await
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ChangeFleetError(#[from] pub FleetChangeFleetError);
