use crate::cmd::shared::{BackrefRenderError, CmdResps, FitIdBackref};

// Commands with full context
pub(in crate::cmd) struct ICmdShipUnsetFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdShipUnsetICtx = ICmdShipUnsetICtx,
}
pub(crate) struct ICmdShipUnsetFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: ICmdShipUnsetICtx,
}

// Commands with incomplete context
pub(crate) struct ICmdShipUnsetICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipUnsetFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdShipUnsetFCtxRIds, BackrefRenderError> {
        Ok(ICmdShipUnsetFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipUnsetFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFitUnsetShipError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitUnsetShipError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdShipUnsetICtx {
    pub(in crate::cmd) fn execute(&self, core_fit: &mut rc::FitMut) {
        if let Some(core_ship) = core_fit.get_ship_mut() {
            core_ship.remove();
        }
    }
}
