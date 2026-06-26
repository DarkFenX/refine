use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HCmdResps, HItemIdsResp,
        shared::{HEffectModeMap, HItemIdBackref},
    },
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HProjEffectChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HProjEffectChangeCmdICtxBIds,
}
pub(crate) struct HProjEffectChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HProjEffectChangeCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
struct HProjEffectChangeCmdICtxBIds {
    #[serde(flatten)]
    shared: HProjEffectChangeCmdShared,
    #[serde(default)]
    add_proj_item_ids: Vec<HItemIdBackref>,
    #[serde(default)]
    rm_proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HProjEffectChangeCmdICtxRIds {
    #[serde(flatten)]
    shared: HProjEffectChangeCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_proj_item_ids: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HProjEffectChangeCmdShared {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HProjEffectChangeCmdFCtxRIds, HExecError> {
        Ok(HProjEffectChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HProjEffectChangeCmdICtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HProjEffectChangeCmdICtxRIds, HExecError> {
        Ok(HProjEffectChangeCmdICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HProjEffectChangeCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_proj_effect = core_sol.get_proj_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetProjEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetProjEffectError::ItemIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.shared.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_proj_effect.set_type_id(core_type_id);
        }
        if let Some(state) = self.shared.state {
            core_proj_effect.set_state(state);
        }
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_proj_effect
                .add_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_proj_effect
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_proj_effect);
        }
        Ok(HItemIdsResp::from_core_proj_effect(core_proj_effect))
    }
}
