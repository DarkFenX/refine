use crate::{
    cmd::HCmdResp,
    shared::HDpsProfile,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFitCmd {
    #[serde_as(as = "TriStateField<serde_with::DisplayFromStr>")]
    #[serde(default)]
    fleet_id: TriStateField<rc::FleetId>,
    sec_status: Option<rc::SecStatus>,
    #[serde(default)]
    rah_incoming_dps: TriStateField<HDpsProfile>,
}
impl HChangeFitCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        match self.fleet_id {
            TriStateField::Value(fleet_id) => {
                if let Err(error) = core_sol.set_fit_fleet(fit_id, &fleet_id) {
                    return Err(match error {
                        rc::err::SetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                        rc::err::SetFitFleetError::FleetNotFound(e) => HExecError::FleetNotFoundSecondary(e),
                    });
                }
            }
            TriStateField::None => {
                if let Err(error) = core_sol.unset_fit_fleet(fit_id) {
                    return Err(match error {
                        rc::err::UnsetFitFleetError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                        rc::err::UnsetFitFleetError::FitHasNoFleet(e) => HExecError::FitIsNotInFleet(e),
                    });
                }
            }
            TriStateField::Absent => (),
        }
        if let Some(sec_status) = self.sec_status {
            if let Err(error) = core_sol.set_fit_sec_status(fit_id, sec_status) {
                return Err(match error {
                    rc::err::SetFitSecStatusError::SecStatusError(e) => HExecError::InvalidSecStatus(e),
                    rc::err::SetFitSecStatusError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        }
        match &self.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => {
                if let Err(error) = core_sol.set_fit_rah_incoming_dps(fit_id, rah_incoming_dps.try_into()?) {
                    return Err(match error {
                        rc::err::SetFitRahIncomingDpsError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                    });
                }
            }
            TriStateField::None => {
                if let Err(error) = core_sol.remove_fit_rah_incoming_dps(fit_id) {
                    match error {
                        rc::err::RemoveFitRahIncomingDpsError::FitNotFound(e) => {
                            return Err(HExecError::FitNotFoundPrimary(e));
                        }
                        // Do nothing if profile was not set
                        rc::err::RemoveFitRahIncomingDpsError::DpsProfileNotSet(_) => (),
                    };
                }
            }
            TriStateField::Absent => (),
        }
        Ok(().into())
    }
}
