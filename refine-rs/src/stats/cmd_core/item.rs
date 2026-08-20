use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{ItemStatsOptions, ItemStatsOptionsBr},
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
