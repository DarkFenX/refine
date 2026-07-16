use crate::{CmdResps, ItemIdBackref, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdItemRemoveFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdItemRemoveICtx = ICmdItemRemoveICtx { .. },
}
pub(crate) struct ICmdItemRemoveFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdItemRemoveICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdItemRemoveICtx {
    pub(in crate::cmd) rm_mode: Option<rc::RemoveMode> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdItemRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdItemRemoveFCtxRIds, BackrefRenderError> {
        Ok(ICmdItemRemoveFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdItemRemoveFCtxRIds {
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
    RemoveFailed(#[from] ItemRemoveItemError),
}

impl ICmdItemRemoveICtx {
    pub(in crate::cmd) fn execute(&self, core_item: rc::ItemMut) -> Result<(), ItemRemoveItemError> {
        core_item.remove(self.rm_mode.unwrap_or(rc::RemoveMode::Free))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemRemoveItemError {
    #[error("{0}")]
    RemoveFailed(#[from] rc::err::RemoveItemError),
}
