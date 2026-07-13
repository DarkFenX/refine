use crate::cmd::{BackrefRenderError, CmdResps, ItemIdBackref};

// Commands with full context
struct CmdItemRemoveFCtxBIds {
    item_id: ItemIdBackref,
    ictx_cmd: CmdItemRemoveICtx,
}
struct CmdItemRemoveFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: CmdItemRemoveICtx,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct CmdItemRemoveICtx {
    pub(in crate::cmd) rm_mode: Option<rc::RmMode>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdItemRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdItemRemoveFCtxRIds, BackrefRenderError> {
        Ok(CmdItemRemoveFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdItemRemoveFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), BasicRemoveItemError> {
        self.ictx_cmd.execute(core_sol, &self.item_id)
    }
}

impl CmdItemRemoveICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<(), BasicRemoveItemError> {
        let core_item = core_sol.get_item_mut(item_id)?;
        core_item.remove(self.rm_mode.unwrap_or(rc::RmMode::Free))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BasicRemoveItemError {
    #[error("{0}")]
    ItemGetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ItemRemoveFailed(#[from] rc::err::RemoveItemError),
}
