use crate::cmd::{fit, shared::HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddModuleCmd,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
