use crate::{CmdResps, ItemId, ItemIdBr, RemoveMode, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemRemoveCmd {
    rm_mode: Option<RemoveMode>,
}

// Extra context commands
pub type ItemRemoveCmdCtxItem = ItemRemoveCmdCtxItemGen<ItemId>;
pub type ItemRemoveCmdCtxItemBr = ItemRemoveCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ItemRemoveCmdCtxItemGen<I> {
    item_id: I,
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
    pub(crate) fn execute(self, core_item: rc::Item) -> Result<(), ItemRemoveError> {
        core_item.remove(self.rm_mode.unwrap_or(RemoveMode::Free))?;
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemRemoveError {
    #[error(transparent)]
    ItemRemove(#[from] rc::err::ItemRemoveError),
}

impl ItemRemoveCmdCtxItem {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), ItemGetItemRemoveError> {
        let core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetItemRemoveError::ItemGet)?;
        Ok(self.core.execute(core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(self, core_fit: &mut rc::Fit) -> Result<(), FitItemGetItemRemoveError> {
        let core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetItemRemoveError::ItemGet)?;
        Ok(self.core.execute(core_item)?)
    }
}

pub type ItemGetItemRemoveError = GenItemGetItemRemoveError<rc::err::ItemGetError>;
pub type FitItemGetItemRemoveError = GenItemGetItemRemoveError<rc::err::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetItemRemoveError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemRemove(rc::err::ItemRemoveError),
}
impl<E> From<ItemRemoveError> for GenItemGetItemRemoveError<E> {
    fn from(err: ItemRemoveError) -> Self {
        match err {
            ItemRemoveError::ItemRemove(inner) => Self::ItemRemove(inner),
        }
    }
}
