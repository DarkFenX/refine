use crate::{
    cmd::{
        shared::{apply_effect_modes, HEffectModeMap},
        HCmdResp,
    },
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::SolItemId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::SolItemId>,
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> HExecResult<HCmdResp> {
        for projectee_item_id in self.add_projs.iter() {
            core_sol.add_proj_effect_proj(item_id, *projectee_item_id)?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_sol.remove_proj_effect_proj(item_id, projectee_item_id)?;
        }
        if let Some(state) = self.state {
            core_sol.set_proj_effect_state(item_id, state)?;
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
