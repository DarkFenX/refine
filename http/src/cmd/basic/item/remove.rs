use serde::Deserialize;

use crate::{
    cmd::shared::{HCmdResps, HItemIdBackref, HRmMode},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HItemRemoveCmdFCtxBIds {
    item_id: HItemIdBackref,
    #[serde(flatten)]
    ictx_cmd: HItemRemoveCmdICtx,
}
pub(crate) struct HItemRemoveCmdFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: HItemRemoveCmdICtx,
}

// Commands with incomplete context
#[derive(Default, Deserialize)]
pub(crate) struct HItemRemoveCmdICtx {
    rm_mode: Option<HRmMode>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemRemoveCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HItemRemoveCmdFCtxRIds, HExecError> {
        Ok(HItemRemoveCmdFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemRemoveCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl HItemRemoveCmdICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<(), HExecError> {
        let core_item = core_sol.get_item_mut(item_id).map_err(|error| match error {
            rc::err::GetItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
        })?;
        core_item
            .remove(self.rm_mode.unwrap_or(HRmMode::Free).into_core())
            .map_err(|error| match error {
                rc::err::RemoveItemError::UnremovableAutocharge => HExecError::UnremovableAutocharge,
            })?;
        Ok(())
    }
}
