use crate::{cmd::HCmdResp, util::HExecResult};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateFleetCmd {}
impl HCreateFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        Ok(core_sol.add_fleet()?.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::SolFleetId,
}
impl HDeleteFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        core_sol.remove_fleet(&self.fleet_id)?;
        Ok(HCmdResp::NoData)
    }
}
