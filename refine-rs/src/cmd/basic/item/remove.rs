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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetItemRemoveItemError> {
        let core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemRemoveItemError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    RemoveFailed(#[from] RemoveItemError),
}

impl CmdItemRemoveICtx {
    pub(in crate::cmd) fn execute(&self, core_item: rc::ItemMut) -> Result<(), RemoveItemError> {
        core_item.remove(self.rm_mode.unwrap_or(rc::RmMode::Free))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveItemError {
    #[error("{0}")]
    RemoveFailed(#[from] rc::err::RemoveItemError),
}
