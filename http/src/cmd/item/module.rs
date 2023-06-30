use crate::{cmd::shared::HEffectModeMap, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HState>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.state.as_ref()
    }
    pub(crate) fn get_effect_modes(&self) -> Option<&HEffectModeMap> {
        self.effect_modes.as_ref()
    }
}
