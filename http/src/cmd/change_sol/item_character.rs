use crate::cmd::{change_fit, HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetCharacterCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolItemId,
    #[serde(flatten)]
    fit_cmd: change_fit::HSetCharacterCmd,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<rc::SolCharacterInfo> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeCharacterCmd {
    ViaItemId(HChangeCharacterViaItemIdCmd),
    ViaFitId(HChangeCharacterViaFitIdCmd),
}
impl HChangeCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeCharacterViaItemIdCmd,
}
impl HChangeCharacterViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_sol)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterViaFitIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeCharacterViaFitIdCmd,
}
impl HChangeCharacterViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}
