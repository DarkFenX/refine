use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HItemIdBackref},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HProjEffectAddCmdFCtxBIds {
    #[serde(flatten)]
    shared: HProjEffectAddCmdShared,
    #[serde(default)]
    proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HProjEffectAddCmdFCtxRIds {
    #[serde(flatten)]
    shared: HProjEffectAddCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HProjEffectAddCmdShared {
    type_id: i32,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HProjEffectAddCmdFCtxRIds, HExecError> {
        Ok(HProjEffectAddCmdFCtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        let core_type_id = rc::ItemTypeId::from_i32(self.shared.type_id);
        let mut core_proj_effect = core_sol.add_proj_effect(core_type_id);
        if let Some(state) = self.shared.state {
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
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_proj_effect);
        }
        Ok(HCreatedItemIdsResp::from_core_proj_effect(core_proj_effect))
    }
}
