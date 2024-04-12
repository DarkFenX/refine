use crate::cmd::HCmdResp;

#[derive(serde::Deserialize)]
pub(crate) struct HCreateFleetCmd {}
impl HCreateFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        Ok(core_ss.add_fleet()?.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::SsFleetId,
}
impl HDeleteFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        core_ss.remove_fleet(&self.fleet_id)?;
        Ok(HCmdResp::NoData)
    }
}
