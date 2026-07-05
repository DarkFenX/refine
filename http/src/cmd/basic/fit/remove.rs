use serde::Deserialize;

use crate::{
    cmd::shared::{HCmdResps, HFitIdBackref, get_primary_fit},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFitRemoveCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFitRemoveCmdICtx,
}
pub(crate) struct HFitRemoveCmdFCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: HFitRemoveCmdICtx,
}

// Commands with incomplete context
#[derive(Default, Deserialize)]
pub(crate) struct HFitRemoveCmdICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitRemoveCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFitRemoveCmdFCtxRIds, HExecError> {
        Ok(HFitRemoveCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitRemoveCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HFitRemoveCmdICtx {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        get_primary_fit(core_sol, fit_id)?.remove();
        Ok(())
    }
}
