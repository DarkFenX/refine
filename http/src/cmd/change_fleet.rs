use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HFleetIdResp, shared::get_primary_fleet},
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_fits: Vec<rc::FitId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    remove_fits: Vec<rc::FitId>,
}
impl HChangeFleetCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<HFleetIdResp, HExecError> {
        let mut core_fleet = get_primary_fleet(core_sol, fleet_id)?;
        for fit_id in self.remove_fits.iter() {
            core_fleet.remove_fit(fit_id).map_err(|error| match error {
                rc::err::FleetRemoveFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetRemoveFitError::FitIsNotInThisFleet(e) => HExecError::FitNotInThisFleet(e),
            })?;
        }
        for fit_id in self.add_fits.iter() {
            core_fleet.add_fit(fit_id).map_err(|error| match error {
                rc::err::FleetAddFitError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                rc::err::FleetAddFitError::FitAlreadyInThisFleet(e) => HExecError::FitAlreadyInThisFleet(e),
            })?;
        }
        Ok(core_fleet.into())
    }
}
