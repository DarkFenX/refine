use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref},
    shared::HServiceState,
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HServiceChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HServiceChangeCmdICtx,
}
pub(crate) struct HServiceChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HServiceChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HServiceChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<HServiceState>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HServiceChangeCmdFCtxRIds, HExecError> {
        Ok(HServiceChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HServiceChangeCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_service = core_sol.get_service_mut(item_id).map_err(|error| match error {
            rc::err::GetServiceError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetServiceError::ItemIsNotService(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_service.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_service.set_state(state.into_core());
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_service);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
