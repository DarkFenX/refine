use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, old_change_item, shared::HEffectModeMap},
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: i32,
    state: Option<bool>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let mut core_proj_effect = core_sol.add_proj_effect(core_type_id);
        if let Some(state) = self.state {
            core_proj_effect.set_state(state);
        }
        for projectee_item_id in self.proj_item_ids.iter() {
            core_proj_effect
                .add_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_proj_effect);
        }
        Ok(HItemIdsResp::from_core_proj_effect(core_proj_effect))
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: old_change_item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
