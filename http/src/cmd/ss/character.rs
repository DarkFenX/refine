use crate::cmd::{fit, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HSetCharacterCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsItemId,
    #[serde(flatten)]
    fit_cmd: fit::HSetCharacterCmd,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeCharacterCmd,
}
impl HChangeCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}
