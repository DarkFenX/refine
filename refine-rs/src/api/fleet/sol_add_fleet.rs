use crate::{AddFleetCmd, Fleet, FleetInfo, FleetInfoMode, SolarSystem, err::AddFleetError};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "flt-add", level = "trace", skip_all)]
    pub async fn add_fleet(&'s mut self, cmd: AddFleetCmd) -> Result<Fleet<'r, 's>, AddFleetError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol).map(|cmd_resp| cmd_resp.fleet_id))
            .await?;
        let fleet = Fleet::new(self, cmd_resp);
        Ok(fleet)
    }
    #[tracing::instrument(name = "flt-add", level = "trace", skip_all)]
    pub async fn add_fleet_and_get_info(
        &'s mut self,
        cmd: AddFleetCmd,
        fleet_mode: FleetInfoMode,
    ) -> Result<(Fleet<'r, 's>, FleetInfo), AddFleetError> {
        let (fleet_id, fleet_info) = self
            .exec_standard_fallible(move |core_sol| {
                let fleet_id = cmd.execute(core_sol).map(|cmd_resp| cmd_resp.fleet_id)?;
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                let item_info = FleetInfo::from_core(&mut core_fleet, fleet_mode);
                Ok::<_, AddFleetError>((fleet_id, item_info))
            })
            .await?;
        let fleet = Fleet::new(self, fleet_id);
        Ok((fleet, fleet_info))
    }
}
