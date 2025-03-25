use crate::{
    cmd::{HCmdResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::ProjEffectInfo {
        core_sol.add_proj_effect(self.type_id, self.state.unwrap_or(true))
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
