use crate::{
    CmdResp, CmdResps, FitIdBr,
    err::BrResolveError,
    trial::{
        FitTryItemsCmdBr,
        core::{FitTryItemsCmdCtxFit, FitTryItemsCmdCtxFitBr},
        err::FitGetFitTryItemsError,
    },
};

#[derive(Clone)]
pub(crate) enum SolTryItemsEnumCmd {
    FitTryItems(FitTryItemsCmdCtxFit),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolTryItemsEnumCmdBr {
    FitTryItems(FitTryItemsCmdCtxFitBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsCmdBr {
    pub fn into_sol_try_br(self, fit_id: impl Into<FitIdBr>) -> SolTryItemsEnumCmdBr {
        SolTryItemsEnumCmdBr::FitTryItems(self.into_ctx_item_br(fit_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolTryItemsEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolTryItemsEnumCmd, BrResolveError> {
        Ok(match self {
            Self::FitTryItems(cmd) => SolTryItemsEnumCmd::FitTryItems(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolTryItemsEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolTryItemsEnumError> {
        Ok(match self {
            Self::FitTryItems(cmd) => CmdResp::TryItems(cmd.execute(core_sol)?),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolTryItemsEnumError {
    #[error(transparent)]
    Fit(#[from] FitGetFitTryItemsError),
}
