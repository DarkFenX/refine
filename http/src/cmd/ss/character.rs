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
#[serde(untagged)]
pub(crate) enum HChangeCharacterCmd {
    ViaItemId(HChangeCharacterViaItemIdCmd),
    ViaFitId(HChangeCharacterViaFitIdCmd),
}
impl HChangeCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_ss),
            Self::ViaFitId(cmd) => cmd.execute(core_ss),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeCharacterViaItemIdCmd,
}
impl HChangeCharacterViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaFitIdCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HChangeCharacterViaFitIdCmd,
}
impl HChangeCharacterViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}
