use crate::{Fleet, FleetAddCmd, FleetInfo, SolarSystem, err::FleetAddError, info::FleetInfoCmd, shared::SolBackup};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "flt-add", level = "trace", skip_all)]
    pub async fn add_fleet(&'s mut self, ctl_cmd: FleetAddCmd) -> Result<Fleet<'r, 's>, FleetAddError> {
        let ctl_cmd_resp = self
            .exec_standard(SolBackup::Needed, move |core_sol| {
                ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.fleet_id)
            })
            .await?;
        let fleet = Fleet::new(self, ctl_cmd_resp);
        Ok(fleet)
    }
    #[tracing::instrument(name = "flt-add-inf", level = "trace", skip_all)]
    pub async fn add_fleet_and_get_info(
        &'s mut self,
        ctl_cmd: FleetAddCmd,
        info_cmd: FleetInfoCmd,
    ) -> Result<(Fleet<'r, 's>, FleetInfo), FleetAddError> {
        let (fleet_id, fleet_info) = self
            .exec_standard(SolBackup::Needed, move |core_sol| {
                let fleet_id = ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.fleet_id)?;
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                let fleet_info = info_cmd.execute(&mut core_fleet);
                Ok::<_, FleetAddError>((fleet_id, fleet_info))
            })
            .await?;
        let fleet = Fleet::new(self, fleet_id);
        Ok((fleet, fleet_info))
    }
}
