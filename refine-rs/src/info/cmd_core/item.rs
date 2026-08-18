use crate::{
    CmdResps, ItemId, ItemIdBr, ItemInfo, ItemInfoMode,
    err::BrResolveError,
    info::{InfoModes, InfoModesCompact},
    shared::BrResolvable,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModes<ItemInfoMode, ItemId>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModesCompact<ItemInfoMode, ItemIdBr>,
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
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        for item_id in item_ids {
            self.item.overrides.insert(item_id, mode);
        }
        self
    }
}

impl ItemInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ItemInfoCmd, BrResolveError> {
        Ok(ItemInfoCmd {
            item: InfoModes::from_compact_br(self.item, resps)?,
        })
    }
}

impl ItemInfoCmdCtxItemBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<ItemInfoCmdCtxItem, BrResolveError> {
        Ok(ItemInfoCmdCtxItem {
            item_id: self.item_id.br_resolve(resps)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemInfo {
        ItemInfo::from_core(core_item, &self.item)
    }
}
