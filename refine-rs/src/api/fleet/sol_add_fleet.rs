use crate::{
    Fleet, Sol,
    ctl::{FleetAddCmd, err::FleetAddError},
    info::{FleetInfo, FleetInfoCmd},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'refine, 'sol> Sol<'refine> {
    #[tracing::instrument(name = "flt-add", level = "error", skip_all)]
    pub async fn add_fleet(&'sol mut self, ctl_cmd: FleetAddCmd) -> Result<Fleet<'refine, 'sol>, FleetAddError> {
        let ctl_cmd_resp = self
            .exec_standard(|core_sol| ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.fleet_id))
            .await?;
        let fleet = Fleet::new(self, ctl_cmd_resp);
        Ok(fleet)
    }
    #[tracing::instrument(name = "flt-add-inf", level = "error", skip_all)]
    pub async fn add_fleet_and_get_info(
        &'sol mut self,
        ctl_cmd: FleetAddCmd,
        info_cmd: FleetInfoCmd,
    ) -> Result<(Fleet<'refine, 'sol>, FleetInfo), FleetAddError> {
        let (fleet_id, fleet_info) = self
            .exec_standard(|core_sol| {
                let fleet_id = ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.fleet_id)?;
                let mut core_fleet = core_sol.get_fleet(&fleet_id).unwrap();
                let fleet_info = info_cmd.execute(&mut core_fleet);
                Ok::<_, FleetAddError>((fleet_id, fleet_info))
            })
            .await?;
        let fleet = Fleet::new(self, fleet_id);
        Ok((fleet, fleet_info))
    }
}
