use crate::cmd::HCmdResp;

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetFleetCmd {
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    fleet_id: Option<rc::SsFleetId>,
}
impl HSetFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        core_ss.set_fit_fleet(fit_id, self.fleet_id)?;
        Ok(HCmdResp::NoData)
    }
}
