use crate::{CmdResps, FitIdBackref, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdFitRemoveFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFitRemoveICtx = ICmdFitRemoveICtx,
}
pub(crate) struct ICmdFitRemoveFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: ICmdFitRemoveICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdFitRemoveICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFitRemoveFCtxRIds, BackrefRenderError> {
        Ok(ICmdFitRemoveFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFitRemoveFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitRemoveFitError> {
        let core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitRemoveFitError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdFitRemoveICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: rc::FitMut) {
        core_fit.remove()
    }
}
