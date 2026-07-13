use crate::cmd::{BackrefRenderError, CmdResps, FitIdBackref};

// Commands with full context
struct CmdFitRemoveFCtxBIds {
    fit_id: FitIdBackref,
    ictx_cmd: CmdFitRemoveICtx,
}
struct CmdFitRemoveFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: CmdFitRemoveICtx,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct CmdFitRemoveICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFitRemoveFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdFitRemoveFCtxRIds, BackrefRenderError> {
        Ok(CmdFitRemoveFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdFitRemoveFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetRemoveFitError> {
        let core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRemoveFitError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl CmdFitRemoveICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: rc::FitMut) {
        core_fit.remove()
    }
}
