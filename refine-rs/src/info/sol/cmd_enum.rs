use crate::{
    CmdResp, CmdResps, FitIdBr, FitInfoCmdBr, FleetIdBr, FleetInfoCmdBr, ItemIdBr, ItemInfoCmdBr, SolInfoCmd,
    SolInfoCmdBr,
    err::{BrResolveError, FitGetFitInfoError, FleetGetFleetInfoError, ItemGetItemInfoError},
    info::{
        fit::{FitInfoCmdCtxFit, FitInfoCmdCtxFitBr},
        fleet::{FleetInfoCmdCtxFleet, FleetInfoCmdCtxFleetBr},
        item::{ItemInfoCmdCtxItem, ItemInfoCmdCtxItemBr},
    },
    svc::SolCtx,
};

#[expect(clippy::enum_variant_names)]
#[derive(Clone)]
pub(crate) enum SolInfoEnumCmd {
    SolInfo(SolInfoCmd),
    FleetInfo(FleetInfoCmdCtxFleet),
    FitInfo(FitInfoCmdCtxFit),
    ItemInfo(ItemInfoCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolInfoEnumCmdBr {
    SolInfo(SolInfoCmdBr),
    FleetInfo(FleetInfoCmdCtxFleetBr),
    FitInfo(FitInfoCmdCtxFitBr),
    ItemInfo(ItemInfoCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl SolInfoCmdBr {
    pub(crate) fn into_sol_info_br(self) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::SolInfo(self)
    }
}
impl FleetInfoCmdBr {
    pub(crate) fn into_sol_info_br(self, fleet_id: impl Into<FleetIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::FleetInfo(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FitInfoCmdBr {
    pub(crate) fn into_sol_info_br(self, fit_id: impl Into<FitIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::FitInfo(self.into_ctx_fit_br(fit_id))
    }
}
impl ItemInfoCmdBr {
    pub(crate) fn into_sol_info_br(self, item_id: impl Into<ItemIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::ItemInfo(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolInfoEnumCmd, BrResolveError> {
        Ok(match self {
            Self::SolInfo(cmd) => SolInfoEnumCmd::SolInfo(cmd.br_resolve(resps)),
            Self::FleetInfo(cmd) => SolInfoEnumCmd::FleetInfo(cmd.br_resolve(resps)?),
            Self::FitInfo(cmd) => SolInfoEnumCmd::FitInfo(cmd.br_resolve(resps)?),
            Self::ItemInfo(cmd) => SolInfoEnumCmd::ItemInfo(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoEnumCmd {
    pub(crate) fn execute(self, ctx: SolCtx, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolInfoEnumError> {
        Ok(match self {
            Self::SolInfo(cmd) => cmd.execute(ctx.sol_id, ctx.src_alias, core_sol).into(),
            Self::FleetInfo(cmd) => cmd.execute(core_sol)?.into(),
            Self::FitInfo(cmd) => cmd.execute(core_sol)?.into(),
            Self::ItemInfo(cmd) => cmd.execute(core_sol)?.into(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolInfoEnumError {
    #[error(transparent)]
    Fleet(#[from] FleetGetFleetInfoError),
    #[error(transparent)]
    Fit(#[from] FitGetFitInfoError),
    #[error(transparent)]
    Item(#[from] ItemGetItemInfoError),
}
