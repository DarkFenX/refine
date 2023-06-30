use crate::cmd::{fit, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddBoosterCmd,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
