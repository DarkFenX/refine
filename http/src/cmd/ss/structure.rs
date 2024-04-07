use crate::cmd::{fit, HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetStructureCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HSetStructureCmd,
}
impl HSetStructureCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStructureCmd {
    ViaItemId(HChangeStructureViaItemIdCmd),
    ViaFitId(HChangeStructureViaFitIdCmd),
}
impl HChangeStructureCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_ss),
            Self::ViaFitId(cmd) => cmd.execute(core_ss),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStructureViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeStructureViaItemIdCmd,
}
impl HChangeStructureViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeStructureViaFitIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HChangeStructureViaFitIdCmd,
}
impl HChangeStructureViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}
