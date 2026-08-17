use crate::{CmdResps, ItemId, ItemIdBr, RemoveMode, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemRemoveCmd {
    rm_mode: Option<RemoveMode>,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemRemoveCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemRemoveCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemRemoveCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemRemoveCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemRemoveCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_rm_mode(mut self, rm_mode: RemoveMode) -> Self {
        self.rm_mode = Some(rm_mode);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemRemoveCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> ItemRemoveCmdCtxItem {
        ItemRemoveCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ItemRemoveCmdCtxItemBr {
        ItemRemoveCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemRemoveCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ItemRemoveCmdCtxItem, BrResolveError> {
        Ok(ItemRemoveCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemRemoveCmd {
    pub(crate) fn execute(self, core_item: rc::ItemMut) -> Result<(), ItemRemoveError> {
        core_item.remove(self.rm_mode.unwrap_or(RemoveMode::Free))?;
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemRemoveError {
    #[error(transparent)]
    ItemRemove(#[from] rc::err::RemoveItemError),
}

impl ItemRemoveCmdCtxItem {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), ItemGetItemRemoveError> {
        let core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetItemRemoveError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemRemove(rc::err::RemoveItemError),
}
impl From<ItemRemoveError> for ItemGetItemRemoveError {
    fn from(err: ItemRemoveError) -> Self {
        match err {
            ItemRemoveError::ItemRemove(inner) => Self::ItemRemove(inner),
        }
    }
}
