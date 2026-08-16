use crate::{CtlCmdResps, FitId, FitIdBr, err::BackrefRenderError};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitRemoveCmd;

// Full context commands
pub struct FitRemoveCmdCtxFit {
    fit_id: FitId,
    core: FitRemoveCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FitRemoveCmdCtxFitBr {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) core: FitRemoveCmd = FitRemoveCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmd {
    pub fn new() -> Self {
        Self::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmdCtxFitBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<FitRemoveCmdCtxFit, BackrefRenderError> {
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
