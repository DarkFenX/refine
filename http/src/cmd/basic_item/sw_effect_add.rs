use serde::Deserialize;

use crate::cmd::{HItemIdsResp, shared::HEffectModeMap};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HSwEffectAddCmdFCtx {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectAddCmdFCtx {
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
