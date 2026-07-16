use crate::{CmdResps, FitIdBackref, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdStanceUnsetFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdStanceUnsetICtx = ICmdStanceUnsetICtx,
}
pub(crate) struct ICmdStanceUnsetFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: ICmdStanceUnsetICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdStanceUnsetICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdStanceUnsetFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdStanceUnsetFCtxRIds, BackrefRenderError> {
        Ok(ICmdStanceUnsetFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdStanceUnsetFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitUnsetStanceError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitUnsetStanceError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdStanceUnsetICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) {
        if let Some(core_stance) = core_fit.get_stance_mut() {
            core_stance.remove();
        }
    }
}
