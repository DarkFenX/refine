use crate::{
    cmd::{change_fit, shared::HCmdResp},
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddModuleCmd,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<rc::SolModuleInfo> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.fit_cmd.execute(core_sol)
    }
}
