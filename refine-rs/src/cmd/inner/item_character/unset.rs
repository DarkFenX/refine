use crate::{CmdResps, FitIdBackref, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdCharacterUnsetFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdCharacterUnsetICtx = ICmdCharacterUnsetICtx,
}
pub(crate) struct ICmdCharacterUnsetFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: ICmdCharacterUnsetICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdCharacterUnsetICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterUnsetFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdCharacterUnsetFCtxRIds, BackrefRenderError> {
        Ok(ICmdCharacterUnsetFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterUnsetFCtxRIds {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitUnsetCharacterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitUnsetCharacterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdCharacterUnsetICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) {
        if let Some(core_character) = core_fit.get_character_mut() {
            core_character.remove();
        }
    }
}
