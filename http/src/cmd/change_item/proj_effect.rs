use crate::cmd::{
    shared::{apply_effect_modes, HEffectModeMap},
    HCmdResp,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_tgts: Vec<rc::SsItemId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_tgts: Vec<rc::SsItemId>,
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        for tgt_item_id in self.add_tgts.iter() {
            core_ss.add_proj_effect_tgt(item_id, *tgt_item_id)?;
        }
        for tgt_item_id in self.rm_tgts.iter() {
            core_ss.remove_proj_effect_tgt(item_id, tgt_item_id)?;
        }
        if let Some(state) = self.state {
            core_ss.set_proj_effect_state(item_id, state)?;
        }
        apply_effect_modes(core_ss, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
