use crate::{
    cmd::{HCmdResp, change_fit},
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetStanceCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HSetStanceCmd,
}
impl HSetStanceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolStanceInfo, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStanceCmd {
    ViaItemId(HChangeStanceViaItemIdCmd),
    ViaFitId(HChangeStanceViaFitIdCmd),
}
impl HChangeStanceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeStanceViaItemIdCmd,
}
impl HChangeStanceViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaFitIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeStanceViaFitIdCmd,
}
impl HChangeStanceViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}
