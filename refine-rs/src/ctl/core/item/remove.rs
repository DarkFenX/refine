use crate::{CtlCmdResps, ItemId, ItemIdBr, RemoveMode, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdItemRemoveFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdItemRemoveICtx = ICmdItemRemoveICtx { .. },
}
pub(crate) struct ICmdItemRemoveFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdItemRemoveICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdItemRemoveICtx {
    pub(in crate::ctl) rm_mode: Option<RemoveMode> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdItemRemoveFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdItemRemoveFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetItemRemoveItemError> {
        let core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemRemoveItemError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemRemove(rc::err::RemoveItemError),
}
impl From<ItemRemoveItemError> for GetItemRemoveItemError {
    fn from(err: ItemRemoveItemError) -> Self {
        match err {
            ItemRemoveItemError::ItemRemove(inner) => Self::ItemRemove(inner),
        }
    }
}

impl ICmdItemRemoveICtx {
    pub(in crate::ctl) fn execute(self, core_item: rc::ItemMut) -> Result<(), ItemRemoveItemError> {
        core_item.remove(self.rm_mode.unwrap_or(RemoveMode::Free))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemRemoveItemError {
    #[error(transparent)]
    ItemRemove(#[from] rc::err::RemoveItemError),
}
