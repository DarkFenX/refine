use crate::cmd::{
    shared::{apply_effect_modes, HEffectModeMap},
    HCmdResp,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    level: Option<rc::SkillLevel>,
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(level) = self.level {
            core_sol.set_skill_level(item_id, level)?;
        }
        if let Some(state) = self.state {
            core_sol.set_skill_state(item_id, state)?;
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
