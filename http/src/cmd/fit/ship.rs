use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .set_fit_ship(*fit_id, self.type_id, self.state.unwrap_or(true))?
            .into())
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_ss),
            Self::ViaFitId(cmd) => cmd.execute(core_ss, fit_id),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeShipCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: item::HChangeShipCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        let item_id = core_ss.get_fit_ship_info(fit_id)?.id;
        self.item_cmd.execute(core_ss, &item_id)
    }
}
