use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{ItemStatsOptions, ItemStatsOptionsBr, ItemStatsResp},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: ItemStatsOptions,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemStatsCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: ItemStatsOptionsBr,
}

// Extra context commands
#[derive(Clone)]
pub struct ItemStatsCmdCtxItem {
    item_id: ItemId,
    core: ItemStatsCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ItemStatsCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ItemStatsCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item(mut self, options: ItemStatsOptions) -> Self {
        self.item_options = options;
        self
    }
}

impl ItemStatsCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item(mut self, options: ItemStatsOptionsBr) -> Self {
        self.item_options = options;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ItemStatsCmd, BrResolveError> {
        Ok(ItemStatsCmd {
            item_options: self.item_options.br_resolve(resps)?,
        })
    }
}

impl ItemStatsCmdCtxItemBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<ItemStatsCmdCtxItem, BrResolveError> {
        Ok(ItemStatsCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemStatsResp {
        let resolved_options = self.item_options.stat_resolve();
        let item_result = resolved_options.execute(core_item);
        ItemStatsResp { item: item_result }
    }
}

impl ItemStatsCmdCtxItem {
    fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<ItemStatsResp, ItemGetItemStatsError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetItemStatsError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
}
