use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecResult,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<rc::SolProjEffectInfo> {
        let info = core_sol.add_proj_effect(self.type_id, self.state.unwrap_or(true))?;
        Ok(info)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
