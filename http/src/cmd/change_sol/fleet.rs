use crate::{
    cmd::{HCmdResp, change_fleet},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFleetCmd {}
impl HAddFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HCmdResp {
        let core_fleet = core_sol.add_fleet();
        core_fleet.into()
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::FleetId,
    #[serde(flatten)]
    fleet_cmd: change_fleet::HChangeFleetCmd,
}
impl HChangeFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        Ok(self.fleet_cmd.execute(core_sol, &self.fleet_id)?.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::FleetId,
}
impl HDeleteFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match core_sol.remove_fleet(&self.fleet_id) {
            Ok(_) => Ok(HCmdResp::NoData),
            Err(error) => Err(match error {
                rc::err::RemoveFleetError::FleetNotFound(e) => HExecError::FleetNotFoundPrimary(e),
            }),
        }
    }
}
