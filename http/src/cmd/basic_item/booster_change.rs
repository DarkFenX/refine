use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HCmdResps, HItemIdsResp,
        shared::{HEffectModeMap, HItemIdBackref, HSideEffectMap},
    },
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HBoosterChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HBoosterChangeCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HBoosterChangeCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    ictx_cmd: HBoosterChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HBoosterChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    side_effects: Option<HSideEffectMap>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HBoosterChangeCmdFCtxRIds, HExecError> {
        Ok(HBoosterChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HBoosterChangeCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_booster = core_sol.get_booster_mut(item_id).map_err(|error| match error {
            rc::err::GetBoosterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetBoosterError::ItemIsNotBooster(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_booster.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        if let Some(h_side_effects) = self.side_effects.as_ref() {
            h_side_effects.apply(&mut core_booster);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_booster);
        }
        Ok(HItemIdsResp::from_core_booster(core_booster))
    }
}
