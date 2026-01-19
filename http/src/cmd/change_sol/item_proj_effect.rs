use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_item},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: i32,
    state: Option<bool>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HItemIdsResp {
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_proj_effect = core_sol.add_proj_effect(core_type_id);
        if let Some(state) = self.state {
            core_proj_effect.set_state(state);
        }
        HItemIdsResp::from_core_proj_effect(core_proj_effect)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
