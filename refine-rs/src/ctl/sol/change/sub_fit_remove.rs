use crate::{CtlCmdResps, FitId, FitIdBackref, FitRemoveCmd, SolCtlCmd, err::BackrefRenderError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolCtlFitRemoveCmdBackref {
    fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitRemoveCmd = FitRemoveCmd,
}
pub struct SolCtlFitRemoveCmd {
    fit_id: FitId,
    core: FitRemoveCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitRemoveCmd {
    pub fn into_sol_ctl(self, fit_id: FitIdBackref) -> SolCtlCmd {
        SolCtlCmd::RemoveFit(SolCtlFitRemoveCmdBackref { fit_id, core: self })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlFitRemoveCmdBackref {
    pub(in crate::ctl::sol) fn render(self, resps: &CtlCmdResps) -> Result<SolCtlFitRemoveCmd, BackrefRenderError> {
        Ok(SolCtlFitRemoveCmd {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlFitRemoveCmd {
    pub(in crate::ctl::sol) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), SolCtlFitRemoveError> {
        let core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        self.core.execute(core_fit);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolCtlFitRemoveError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
