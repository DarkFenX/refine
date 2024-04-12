use crate::cmd::{item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        Ok(core_ss
            .add_proj_effect(self.type_id, self.state.unwrap_or(true))?
            .into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
