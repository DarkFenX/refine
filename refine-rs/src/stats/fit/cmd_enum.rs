use crate::{
    CmdResp, CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::CmdResidue,
    stats::{FitStatsCmdBr, FitStatsCmdGen, ItemStatsCmdBr, err::ItemGetItemStatsError, item::ItemStatsCmdCtxItemGen},
};

pub(crate) type FitStatsEnumCmd = FitStatsEnumCmdGen<FitId, ItemId>;
pub type FitStatsEnumCmdBr = FitStatsEnumCmdGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case"),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub enum FitStatsEnumCmdGen<F, I> {
    FitStats(FitStatsCmdGen<F, I>),
    ItemStats(ItemStatsCmdCtxItemGen<F, I>),
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
impl FitStatsEnumCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            Self::FitStats(cmd) => cmd.exec_residue(),
            Self::ItemStats(cmd) => cmd.exec_residue(),
        }
    }
}

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
