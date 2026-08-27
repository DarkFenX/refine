use crate::{AddedItemIdsResp, EffectId, EffectMode, ItemTypeId, ctl::core::shared::EffectModes, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SwEffectAddCmd {
    type_id: ItemTypeId,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectAddCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self { type_id, .. }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectAddCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}

impl SwEffectAddCmd {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> AddedItemIdsResp {
        let mut core_sw_effect = core_sol.add_sw_effect(self.type_id);
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        self.effect_modes.apply(&mut core_sw_effect);
        AddedItemIdsResp::from_core_sw_effect(core_sw_effect)
    }
}
