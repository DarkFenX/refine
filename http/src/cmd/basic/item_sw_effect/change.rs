use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HSwEffectChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HSwEffectChangeCmdICtx,
}
pub(crate) struct HSwEffectChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HSwEffectChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HSwEffectChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HSwEffectChangeCmdFCtxRIds, HExecError> {
        Ok(HSwEffectChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HSwEffectChangeCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_sw_effect = core_sol.get_sw_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetSwEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSwEffectError::ItemIsNotSwEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_sw_effect.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_sw_effect);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
