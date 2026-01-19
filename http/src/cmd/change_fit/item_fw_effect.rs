use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddFwEffectCmd {
    type_id: i32,
    state: Option<bool>,
}
impl HAddFwEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_fw_effect = core_fit.add_fw_effect(core_type_id);
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        Ok(HItemIdsResp::from_core_fw_effect(core_fw_effect))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeFwEffectCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFwEffectCmd,
}
impl HChangeFwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
