use crate::{
    CmdResp, CmdResps, FitIdBr,
    err::BrResolveError,
    svc::SolCtx,
    val::{
        FitValCmdBr, SolValCmd, SolValCmdBr,
        cmd_core::{FitValCmdCtxFit, FitValCmdCtxFitBr},
    },
};

#[expect(clippy::enum_variant_names)]
#[derive(Clone)]
pub(crate) enum SolValEnumCmd {
    SolValidate(SolValCmd),
    FitValidate(FitValCmdCtxFit),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolValEnumCmdBr {
    SolValidate(SolValCmdBr),
    FitValidate(FitValCmdCtxFitBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl SolValCmdBr {
    pub(crate) fn into_sol_val_br(self) -> SolValEnumCmdBr {
        SolValEnumCmdBr::SolValidate(self)
    }
}
impl FitValCmdBr {
    pub(crate) fn into_sol_val_br(self, fit_id: impl Into<FitIdBr>) -> SolValEnumCmdBr {
        SolValEnumCmdBr::FitValidate(self.into_ctx_item_br(fit_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
// impl SolValEnumCmdBr {
//     pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolValEnumCmd, BrResolveError> {
//         Ok(match self {
//             Self::SolValidate(cmd) => SolValEnumCmd::SolValidate(cmd.br_resolve(resps)?),
//             Self::FitValidate(cmd) => SolValEnumCmd::FitValidate(cmd.br_resolve(resps)?),
//         })
//     }
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
// impl SolInfoEnumCmd {
//     pub(crate) fn execute(self, ctx: SolCtx, core_sol: &mut rc::SolarSystem) -> Result<CmdResp,
// SolInfoEnumError> {         Ok(match self {
//             Self::SolInfo(cmd) => cmd.execute(ctx.sol_id, ctx.src_alias, core_sol).into(),
//             Self::FleetInfo(cmd) => cmd.execute(core_sol)?.into(),
//             Self::FitInfo(cmd) => cmd.execute(core_sol)?.into(),
//             Self::ItemInfo(cmd) => cmd.execute(core_sol)?.into(),
//         })
//     }
// }
//
// #[derive(thiserror::Error, Debug)]
// pub enum SolInfoEnumError {
//     #[error(transparent)]
//     Fleet(#[from] FleetGetFleetInfoError),
//     #[error(transparent)]
//     Fit(#[from] FitGetFitInfoError),
//     #[error(transparent)]
//     Item(#[from] ItemGetItemInfoError),
// }
