use crate::{CmdResps, FitId, FitIdBr, err::BrResolveError, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct StanceUnsetCmd;

// Extra context commands
pub type StanceUnsetCmdCtxFit = StanceUnsetCmdCtxFitGen<FitId>;
pub type StanceUnsetCmdCtxFitBr = StanceUnsetCmdCtxFitGen<FitIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct StanceUnsetCmdCtxFitGen<F> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: StanceUnsetCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceUnsetCmd {
    pub fn new() -> Self {
        Self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceUnsetCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> StanceUnsetCmdCtxFit {
        StanceUnsetCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> StanceUnsetCmdCtxFitBr {
        StanceUnsetCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceUnsetCmdCtxFitBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<StanceUnsetCmdCtxFit, BrResolveError> {
        Ok(StanceUnsetCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StanceUnsetCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}
impl<F> StanceUnsetCmdCtxFitGen<F> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl StanceUnsetCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) {
        if let Some(core_stance) = core_fit.get_stance_mut() {
            core_stance.remove();
        }
    }
}

impl StanceUnsetCmdCtxFit {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FitGetStanceUnsetError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        self.core.execute(&mut core_fit);
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetStanceUnsetError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
