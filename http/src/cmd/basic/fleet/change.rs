use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{HCmdResps, HFitIdBackref, HFleetIdBackref, get_primary_fleet},
    err::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HFleetChangeCmdFCtxBIds {
    fleet_id: HFleetIdBackref,
    #[serde(flatten)]
    ictx_cmd: HFleetChangeCmdICtxBIds,
}
pub(crate) struct HFleetChangeCmdFCtxRIds {
    fleet_id: rc::FleetId,
    ictx_cmd: HFleetChangeCmdICtxRIds,
}

// Commands with incomplete context
#[serde_as]
#[derive(Deserialize)]
struct HFleetChangeCmdICtxBIds {
    #[serde(default)]
    add_fit_ids: Vec<HFitIdBackref>,
    #[serde(default)]
    rm_fit_ids: Vec<HFitIdBackref>,
}
#[serde_as]
#[derive(Default, Deserialize)]
pub(crate) struct HFleetChangeCmdICtxRIds {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_fit_ids: Vec<rc::FitId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetChangeCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFleetChangeCmdFCtxRIds, HExecError> {
        Ok(HFleetChangeCmdFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HFleetChangeCmdICtxBIds {
    fn render(self, resps: &HCmdResps) -> Result<HFleetChangeCmdICtxRIds, HExecError> {
        Ok(HFleetChangeCmdICtxRIds {
            add_fit_ids: resps.render_fit_ids(self.add_fit_ids)?,
            rm_fit_ids: resps.render_fit_ids(self.rm_fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFleetChangeCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fleet_id)
    }
}

impl HFleetChangeCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<(), HExecError> {
        let mut core_fleet = get_primary_fleet(core_sol, fleet_id)?;
        for fit_id in self.rm_fit_ids.iter() {
            core_fleet.remove_fit(fit_id).map_err(|error| match error {
                rc::err::FleetRemoveFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetRemoveFitError::FitIsNotInThisFleet(e) => HExecError::FitNotInThisFleet(e),
            })?;
        }
        for fit_id in self.add_fit_ids.iter() {
            core_fleet.add_fit(fit_id).map_err(|error| match error {
                rc::err::FleetAddFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetAddFitError::FitAlreadyInThisFleet(e) => HExecError::FitAlreadyInThisFleet(e),
            })?;
        }
        Ok(())
    }
}
