use crate::{CmdResps, FitId, FitIdBr, err::BrResolveError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitRemoveCmd;

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitRemoveCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitRemoveCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitRemoveCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitRemoveCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmd {
    pub fn new() -> Self {
        Self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmd {
    pub(in crate::ctl) fn into_ctx_fit(self, fit_id: FitId) -> FitRemoveCmdCtxFit {
        FitRemoveCmdCtxFit { fit_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> FitRemoveCmdCtxFitBr {
        FitRemoveCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmdCtxFitBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<FitRemoveCmdCtxFit, BrResolveError> {
        Ok(FitRemoveCmdCtxFit {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmd {
    pub(crate) fn execute(self, core_fit: rc::FitMut) {
        core_fit.remove()
    }
}

impl FitRemoveCmdCtxFit {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FitGetFitRemoveError> {
        let core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        self.core.execute(core_fit);
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitRemoveError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
