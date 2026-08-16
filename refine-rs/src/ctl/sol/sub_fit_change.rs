use crate::{
    CtlCmdResps, FitChangeCmd, FitChangeCmdBr, FitId, FitIdBr,
    err::{BackrefRenderError, FitChangeError},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolCtlFitChangeCmdBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitChangeCmdBr,
}
pub struct SolCtlFitChangeCmd {
    fit_id: FitId,
    core: FitChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlFitChangeCmdBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<SolCtlFitChangeCmd, BackrefRenderError> {
        Ok(SolCtlFitChangeCmd {
            fit_id: resps.render_fit_id(self.fit_id)?,
            core: self.core.render(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlFitChangeCmd {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), SolCtlFitChangeError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolCtlFitChangeError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("failed to set fleet")]
    FleetSet(#[source] rc::err::SetFitFleetError),
}
impl From<FitChangeError> for SolCtlFitChangeError {
    fn from(err: FitChangeError) -> Self {
        match err {
            FitChangeError::FleetSet(inner) => Self::FleetSet(inner),
        }
    }
}
