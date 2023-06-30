use crate::{
    cmd::{shared::HEffectModeMap, HCmdResp},
    shared::HState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HState>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = &self.state {
            core_ss.set_module_state(item_id, state.into())?;
        }
        if let Some(mode_map) = &self.effect_modes {
            let mode_map = mode_map.into_iter().map(|(k, v)| (*k, v.into())).collect();
            core_ss.set_item_effect_modes(item_id, &mode_map)?;
        }
        Ok(HCmdResp::NoData)
    }
}
