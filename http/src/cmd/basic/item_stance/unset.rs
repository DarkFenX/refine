use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HFitIdBackref, get_primary_fit},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HStanceUnsetCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HStanceUnsetCmdICtx,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HStanceUnsetCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HStanceUnsetCmdICtx,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HStanceUnsetCmdICtx;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceUnsetCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HStanceUnsetCmdFCtxRIds, HExecError> {
        Ok(HStanceUnsetCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceUnsetCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HStanceUnsetCmdICtx {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_stance) = core_fit.get_stance_mut() {
            core_stance.remove();
        }
        Ok(())
    }
}
