use crate::{
    cmd::{
        shared::{apply_effect_modes, apply_side_effects, HEffectModeMap, HSideEffectMap},
        HCmdResp,
    },
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    side_effects: Option<HSideEffectMap>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> HExecResult<HCmdResp> {
        if let Some(state) = self.state {
            core_sol.set_booster_state(item_id, state)?;
        }
        apply_side_effects(core_sol, item_id, &self.side_effects)?;
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
