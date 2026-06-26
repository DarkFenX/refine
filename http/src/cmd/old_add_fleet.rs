use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{cmd::HFleetIdResp, util::HExecError};

#[serde_as]
#[derive(Default, Deserialize)]
pub(crate) struct HAddFleetCmd {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    fit_ids: Vec<rc::FitId>,
}
impl HAddFleetCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFleetIdResp, HExecError> {
        let mut core_fleet = core_sol.add_fleet();
        for fit_id in &self.fit_ids {
            core_fleet.add_fit(fit_id).map_err(|error| match error {
                rc::err::FleetAddFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetAddFitError::FitAlreadyInThisFleet(e) => HExecError::FitAlreadyInThisFleet(e),
            })?;
        }
        Ok(HFleetIdResp::from_core_fleet(core_fleet))
    }
}
