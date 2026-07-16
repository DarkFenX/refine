use crate::{AddedItemIdsResp, cmd::shared::EffectModes};

// Commands with full context
pub(crate) struct ICmdSwEffectAddFCtx {
    pub(in crate::cmd) type_id: rc::ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSwEffectAddFCtx {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> AddedItemIdsResp {
        let mut core_sw_effect = core_sol.add_sw_effect(self.type_id);
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        self.effect_modes.apply(&mut core_sw_effect);
        AddedItemIdsResp::from_core_sw_effect(core_sw_effect)
    }
}
