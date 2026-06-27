use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HAutochargeChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HAutochargeChangeCmdICtx,
}
pub(crate) struct HAutochargeChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HAutochargeChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HAutochargeChangeCmdICtx {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAutochargeChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HAutochargeChangeCmdFCtxRIds, HExecError> {
        Ok(HAutochargeChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAutochargeChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HAutochargeChangeCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_autocharge = core_sol.get_autocharge_mut(item_id).map_err(|error| match error {
            rc::err::GetAutochargeError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetAutochargeError::ItemIsNotAutocharge(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_autocharge.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_autocharge);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
