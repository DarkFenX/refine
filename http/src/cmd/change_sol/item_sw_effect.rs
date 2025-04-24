use crate::{
    cmd::{HItemIdsResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSwEffectCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HAddSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HItemIdsResp {
        let mut sw_effect = core_sol.add_sw_effect(self.type_id);
        if let Some(state) = self.state {
            sw_effect.set_state(state);
        }
        sw_effect.into()
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSwEffectCmd,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
