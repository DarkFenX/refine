use crate::{
    cmd::{change_fit, HCmdResp},
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddRigCmd,
}
impl HAddRigCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolRigInfo, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeRigCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeRigCmd,
}
impl HChangeRigCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}
