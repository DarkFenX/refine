use crate::{
    cmd::{
        shared::{apply_effect_modes, HEffectModeMap, HProjDef},
        HCmdResp,
    },
    shared::HState,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDef>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::SolItemId>,
    state: Option<HState>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    charge: Option<Option<rc::EItemId>>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> rc::Result<HCmdResp> {
        for proj_def in self.add_projs.iter() {
            core_sol.add_module_proj(item_id, proj_def.get_item_id(), proj_def.get_range())?;
        }
        for proj_def in self.change_projs.iter() {
            core_sol.change_module_proj(item_id, &proj_def.get_item_id(), proj_def.get_range())?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_sol.remove_module_proj(item_id, projectee_item_id)?;
        }
        if let Some(state) = &self.state {
            core_sol.set_module_state(item_id, state.into())?;
        }
        if let Some(charge_opt) = &self.charge {
            match charge_opt {
                Some(charge) => {
                    core_sol.set_module_charge(item_id, *charge)?;
                }
                None => {
                    core_sol.remove_module_charge(item_id)?;
                }
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
