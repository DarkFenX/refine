use crate::{
    cmd::{
        shared::{apply_effect_modes, HEffectModeMap},
        HCmdResp,
    },
    shared::HState,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HState>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
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
        apply_effect_modes(core_ss, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
