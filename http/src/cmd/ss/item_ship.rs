use crate::cmd::{fit, HCmdResp};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HSetShipCmd,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_ss),
            Self::ViaFitId(cmd) => cmd.execute(core_ss),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeShipViaItemIdCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HChangeShipViaFitIdCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.fit_cmd.execute(core_ss, &self.fit_id)
    }
}
