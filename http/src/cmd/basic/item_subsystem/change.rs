use serde::Deserialize;

use crate::{
    cmd::shared::{HChangedItemIdsResp, HCmdResps, HEffectModeMap, HItemIdBackref},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HSubsystemChangeCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HSubsystemChangeCmdICtx,
}
pub(crate) struct HSubsystemChangeCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HSubsystemChangeCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HSubsystemChangeCmdICtx {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSubsystemChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HSubsystemChangeCmdFCtxRIds, HExecError> {
        Ok(HSubsystemChangeCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSubsystemChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HChangedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HSubsystemChangeCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HChangedItemIdsResp, HExecError> {
        let mut core_subsystem = core_sol.get_subsystem_mut(item_id).map_err(|error| match error {
            rc::err::GetSubsystemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSubsystemError::ItemIsNotSubsystem(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_subsystem.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        };
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_subsystem);
        }
        Ok(HChangedItemIdsResp::default())
    }
}
