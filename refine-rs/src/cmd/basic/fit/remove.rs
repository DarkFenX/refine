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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), BasicRemoveFitError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl CmdFitRemoveICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<(), BasicRemoveFitError> {
        core_sol.get_fit_mut(fit_id)?.remove();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BasicRemoveFitError {
    #[error("{0}")]
    FitGetFailed(#[from] rc::err::GetFitError),
}
