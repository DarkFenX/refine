use crate::{
    CmdResp, CmdResps, FitIdBr, FleetIdBr, ItemIdBr,
    err::BrResolveError,
    stats::{
        FitStatsCmdBr, FleetStatsCmdBr, ItemStatsCmdBr, SolStatsCmd, SolStatsCmdBr,
        err::{FitGetFitStatsError, FleetGetFleetStatsError, ItemGetItemStatsError},
        fit::{FitStatsCmdCtxFit, FitStatsCmdCtxFitBr},
        fleet::{FleetStatsCmdCtxFleet, FleetStatsCmdCtxFleetBr},
        item::{ItemStatsCmdCtxItem, ItemStatsCmdCtxItemBr},
    },
};

#[expect(clippy::enum_variant_names)]
#[derive(Clone)]
pub(crate) enum SolStatsEnumCmd {
    SolStats(SolStatsCmd),
    FleetStats(FleetStatsCmdCtxFleet),
    FitStats(FitStatsCmdCtxFit),
    ItemStats(ItemStatsCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolStatsEnumCmdBr {
    SolStats(SolStatsCmdBr),
    FleetStats(FleetStatsCmdCtxFleetBr),
    FitStats(FitStatsCmdCtxFitBr),
    ItemStats(ItemStatsCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl SolStatsCmdBr {
    pub(crate) fn into_sol_stats_br(self) -> SolStatsEnumCmdBr {
        SolStatsEnumCmdBr::SolStats(self)
    }
}
impl FleetStatsCmdBr {
    pub(crate) fn into_sol_stats_br(self, fleet_id: impl Into<FleetIdBr>) -> SolStatsEnumCmdBr {
        SolStatsEnumCmdBr::FleetStats(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FitStatsCmdBr {
    pub(crate) fn into_sol_stats_br(self, fit_id: impl Into<FitIdBr>) -> SolStatsEnumCmdBr {
        SolStatsEnumCmdBr::FitStats(self.into_ctx_fit_br(fit_id))
    }
}
impl ItemStatsCmdBr {
    pub(crate) fn into_sol_stats_br(self, item_id: impl Into<ItemIdBr>) -> SolStatsEnumCmdBr {
        SolStatsEnumCmdBr::ItemStats(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolStatsEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolStatsEnumCmd, BrResolveError> {
        Ok(match self {
            Self::SolStats(cmd) => SolStatsEnumCmd::SolStats(cmd.br_resolve(resps)),
            Self::FleetStats(cmd) => SolStatsEnumCmd::FleetStats(cmd.br_resolve(resps)?),
            Self::FitStats(cmd) => SolStatsEnumCmd::FitStats(cmd.br_resolve(resps)?),
            Self::ItemStats(cmd) => SolStatsEnumCmd::ItemStats(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolStatsEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolStatsEnumError> {
        Ok(match self {
            Self::SolStats(cmd) => cmd.execute(core_sol).into(),
            Self::FleetStats(cmd) => cmd.execute(core_sol)?.into(),
            Self::FitStats(cmd) => cmd.execute(core_sol)?.into(),
            Self::ItemStats(cmd) => cmd.execute(core_sol)?.into(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolStatsEnumError {
    #[error(transparent)]
    Fleet(#[from] FleetGetFleetStatsError),
    #[error(transparent)]
    Fit(#[from] FitGetFitStatsError),
    #[error(transparent)]
    Item(#[from] ItemGetItemStatsError),
}
