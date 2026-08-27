use crate::{
    CmdResp, CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::CmdResidue,
    val::{FitValCmdBr, SolValCmdBr, SolValCmdGen, err::FitGetFitValError, fit::FitValCmdCtxFitGen},
};

pub(crate) type SolValEnumCmd = SolValEnumCmdGen<FitId, ItemId>;
pub type SolValEnumCmdBr = SolValEnumCmdGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case"),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub enum SolValEnumCmdGen<F, I> {
    SolValidate(SolValCmdGen<F, I>),
    FitValidate(FitValCmdCtxFitGen<F, I>),
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
impl SolValEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolValEnumCmd, BrResolveError> {
        Ok(match self {
            Self::SolValidate(cmd) => SolValEnumCmd::SolValidate(cmd.br_resolve(resps)),
            Self::FitValidate(cmd) => SolValEnumCmd::FitValidate(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValEnumCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            Self::SolValidate(cmd) => cmd.exec_residue(),
            Self::FitValidate(cmd) => cmd.exec_residue(),
        }
    }
}

impl SolValEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolValEnumError> {
        Ok(match self {
            Self::SolValidate(cmd) => cmd.execute(core_sol).into(),
            Self::FitValidate(cmd) => cmd.execute(core_sol)?.into(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolValEnumError {
    #[error(transparent)]
    Fit(#[from] FitGetFitValError),
}
