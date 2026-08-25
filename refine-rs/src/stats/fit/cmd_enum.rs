use crate::{
    CmdResp, CmdResps, ItemIdBr,
    err::BrResolveError,
    stats::{
        FitStatsCmd, FitStatsCmdBr, ItemStatsCmdBr,
        err::ItemGetItemStatsError,
        item::{ItemStatsCmdCtxItem, ItemStatsCmdCtxItemBr},
    },
};

#[derive(Clone)]
pub(crate) enum FitStatsEnumCmd {
    FitStats(FitStatsCmd),
    ItemStats(ItemStatsCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum FitStatsEnumCmdBr {
    FitStats(FitStatsCmdBr),
    ItemStats(ItemStatsCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl FitStatsCmdBr {
    pub(crate) fn into_fit_stats_br(self) -> FitStatsEnumCmdBr {
        FitStatsEnumCmdBr::FitStats(self)
    }
}
impl ItemStatsCmdBr {
    pub(crate) fn into_fit_stats_br(self, item_id: impl Into<ItemIdBr>) -> FitStatsEnumCmdBr {
        FitStatsEnumCmdBr::ItemStats(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<FitStatsEnumCmd, BrResolveError> {
        Ok(match self {
            Self::FitStats(cmd) => FitStatsEnumCmd::FitStats(cmd.br_resolve(resps)),
            Self::ItemStats(cmd) => FitStatsEnumCmd::ItemStats(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CmdResp, FitStatsEnumError> {
        Ok(match self {
            Self::FitStats(cmd) => cmd.execute(core_fit).into(),
            Self::ItemStats(cmd) => cmd.execute(core_fit.get_sol_mut())?.into(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitStatsEnumError {
    #[error(transparent)]
    Item(#[from] ItemGetItemStatsError),
}
