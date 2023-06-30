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
pub(crate) struct HChangeShipCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeShipCmd,
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
