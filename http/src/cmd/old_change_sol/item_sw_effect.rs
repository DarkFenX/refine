use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, old_change_item, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddSwEffectCmd {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HItemIdsResp {
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_sw_effect = core_sol.add_sw_effect(core_type_id);
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_sw_effect);
        }
        HItemIdsResp::from_core_sw_effect(core_sw_effect)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeSwEffectCmd,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
