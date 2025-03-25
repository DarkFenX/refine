use crate::{cmd::HCmdResp, util::HExecError};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetFleetCmd {
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    fleet_id: Option<rc::FleetId>,
}
impl HSetFleetCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        match self.fleet_id {
            Some(fleet_id) => {
                if let Err(error) = core_sol.set_fit_fleet(fit_id, fleet_id) {
                    return Err(match error {
                        rc::err::SetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                        rc::err::SetFitFleetError::FleetNotFound(e) => HExecError::FleetNotFoundSecondary(e),
                    });
                }
            }
            None => {
                if let Err(error) = core_sol.unset_fit_fleet(fit_id) {
                    return Err(match error {
                        rc::err::UnsetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                        rc::err::UnsetFitFleetError::FitHasNoFleet(e) => HExecError::FitIsNotInFleet(e),
                    });
                }
            }
        }

        Ok(HCmdResp::NoData)
    }
}
