use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSwEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        Ok(core_ss.add_sw_effect(self.type_id, self.state.unwrap_or(true))?.into())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeSwEffectCmd,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
