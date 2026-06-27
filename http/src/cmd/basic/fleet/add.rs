use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HCreatedFleetIdResp, HFitIdBackref},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFleetAddCmdFCtxBIds {
    #[serde(default)]
    fit_ids: Vec<HFitIdBackref>,
}
#[serde_as]
#[derive(Default, Deserialize)]
pub(crate) struct HFleetAddCmdFCtxRIds {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFleetAddCmdFCtxRIds, HExecError> {
        Ok(HFleetAddCmdFCtxRIds {
            fit_ids: resps.render_fit_ids(self.fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedFleetIdResp, HExecError> {
        let mut core_fleet = core_sol.add_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id).map_err(|error| match error {
                rc::err::FleetAddFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetAddFitError::FitAlreadyInThisFleet(e) => HExecError::FitAlreadyInThisFleet(e),
            })?;
        }
        Ok(HCreatedFleetIdResp::from_core_fleet(core_fleet))
    }
}
