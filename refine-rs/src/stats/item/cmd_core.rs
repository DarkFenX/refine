use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{BrResolveInfallible, CmdResidue},
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmdBr {
    pub(in crate::stats) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ItemStatsCmdCtxItemBr {
        ItemStatsCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> ItemStatsCmd {
        ItemStatsCmd {
            item_options: self.item_options.br_resolve_infallible(resps),
        }
    }
}

impl ItemStatsCmdCtxItemBr {
    pub(in crate::stats) fn br_resolve(self, resps: &CmdResps) -> Result<ItemStatsCmdCtxItem, BrResolveError> {
        Ok(ItemStatsCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsCmdBr {
    fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl ItemStatsCmdCtxItemBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl ItemStatsCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemStatsResp {
        ItemStatsResp {
            item: self.item_options.stat_resolve().execute(core_item),
        }
    }
}

impl ItemStatsCmdCtxItem {
    pub(in crate::stats) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ItemStatsResp, ItemGetItemStatsError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetItemStatsError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::ItemGetError),
}
