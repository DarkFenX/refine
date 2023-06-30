use crate::cmd::{fit, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSubsystemCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddSubsystemCmd,
}
impl HAddSubsystemCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeSubsystemCmd,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
