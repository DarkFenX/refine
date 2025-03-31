use crate::{
    cmd::HCmdResp,
    shared::HDmgProfile,
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
    rah_incoming_dmg: TriStateField<HDmgProfile>,
}
impl HChangeFitCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        match self.fleet_id {
            TriStateField::Value(fleet_id) => {
                if let Err(error) = core_sol.set_fit_fleet(fit_id, fleet_id) {
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
        match &self.rah_incoming_dmg {
            TriStateField::Value(rah_incoming_dmg) => {
                if let Err(error) = core_sol.set_fit_rah_incoming_dmg(fit_id, rah_incoming_dmg.into()) {
                    return Err(match error {
                        rc::err::SetFitRahIncomingDmgError::EmDmgNegative(e) => HExecError::InvalidDmgProfileEm(e),
                        rc::err::SetFitRahIncomingDmgError::ThermDmgNegative(e) => {
                            HExecError::InvalidDmgProfileTherm(e)
                        }
                        rc::err::SetFitRahIncomingDmgError::KinDmgNegative(e) => HExecError::InvalidDmgProfileKin(e),
                        rc::err::SetFitRahIncomingDmgError::ExplDmgNegative(e) => HExecError::InvalidDmgProfileExpl(e),
                        rc::err::SetFitRahIncomingDmgError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                    });
                }
            }
            TriStateField::None => {
                if let Err(error) = core_sol.remove_fit_rah_incoming_dmg(fit_id) {
                    match error {
                        rc::err::RemoveFitRahIncomingDmgError::FitNotFound(e) => {
                            return Err(HExecError::FitNotFoundPrimary(e));
                        }
                        // Do nothing if profile was not set
                        rc::err::RemoveFitRahIncomingDmgError::DmgProfileNotSet(_) => (),
                    };
                }
            }
            TriStateField::Absent => (),
        }
        Ok(().into())
    }
}
