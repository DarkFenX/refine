use crate::util::HExecError;

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_fits: Vec<rc::FitId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    remove_fits: Vec<rc::FitId>,
}
impl HChangeFleetCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<rc::FleetInfo, HExecError> {
        let core_fleet = match core_sol.get_fleet(fleet_id) {
            Ok(core_fleet) => core_fleet,
            Err(error) => {
                return Err(match error {
                    rc::err::GetFleetError::FleetNotFound(e) => HExecError::FleetNotFoundPrimary(e),
                });
            }
        };
        for fit_id in self.remove_fits.iter() {
            if !core_fleet.fits.contains(fit_id) {
                return Err(HExecError::FitIsNotInThisFleet(*fleet_id, *fit_id));
            }
            if let Err(error) = core_sol.unset_fit_fleet(fit_id) {
                return Err(match error {
                    rc::err::UnsetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                    rc::err::UnsetFitFleetError::FitHasNoFleet(e) => HExecError::FitIsNotInFleet(e),
                });
            }
        }
        for fit_id in self.add_fits.iter() {
            if let Err(error) = core_sol.set_fit_fleet(fit_id, fleet_id) {
                return Err(match error {
                    rc::err::SetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundSecondary(e),
                    // We already checked that fleet exists, this error should never happen
                    rc::err::SetFitFleetError::FleetNotFound(_) => panic!(),
                });
            }
        }
        let info = core_sol.get_fleet(fleet_id).unwrap();
        Ok(info)
    }
}
