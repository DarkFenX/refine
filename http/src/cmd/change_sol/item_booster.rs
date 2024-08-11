use crate::{
    cmd::{change_fit, HCmdResp},
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddBoosterCmd,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolBoosterInfo, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}
