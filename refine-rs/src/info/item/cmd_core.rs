use crate::{
    CmdResps, ItemId, ItemIdBr, ItemInfo, ItemInfoMode,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapLight},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdMapLight<ItemId, ItemInfoMode>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdCompact<ItemIdBr, ItemInfoMode>,
}

// Extra context commands
#[derive(Clone)]
pub struct ItemInfoCmdCtxItem {
    item_id: ItemId,
    core: ItemInfoCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemInfoCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemInfoCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

impl ItemInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmdBr {
    pub(in crate::info) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ItemInfoCmdCtxItemBr {
        ItemInfoCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> ItemInfoCmd {
        ItemInfoCmd {
            item_mode: OvrdMapLight::from_compact_with_br_resolution(self.item_mode, resps),
        }
    }
}

impl ItemInfoCmdCtxItemBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<ItemInfoCmdCtxItem, BrResolveError> {
        Ok(ItemInfoCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemInfo {
        ItemInfo::from_core(core_item, &self.item_mode)
    }
}

impl ItemInfoCmdCtxItem {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<ItemInfo, ItemGetItemInfoError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetItemInfoError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
}
