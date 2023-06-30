use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .add_booster(*fit_id, self.type_id, self.state.unwrap_or(true))?
            .into())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
