use crate::{cmd::HCmdResp, util::HExecResult};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateFitCmd {}
impl HCreateFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        Ok(core_sol.add_fit()?.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
}
impl HDeleteFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        core_sol.remove_fit(&self.fit_id)?;
        Ok(HCmdResp::NoData)
    }
}
