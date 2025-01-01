use crate::{cmd::HCmdResp, shared::HDmgProfile, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HSetRahIncomingDmgCmd {
    dmg_profile: Option<HDmgProfile>,
}
impl HSetRahIncomingDmgCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<HCmdResp, HExecError> {
        match &self.dmg_profile {
            Some(dmg_profile) => {
                if let Err(error) = core_sol.set_fit_rah_incoming_dmg(fit_id, dmg_profile.into()) {
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
            None => {
                if let Err(error) = core_sol.remove_fit_rah_incoming_dmg(fit_id) {
                    match error {
                        rc::err::RemoveFitRahIncomingDmgError::FitNotFound(e) => {
                            return Err(HExecError::FitNotFoundPrimary(e))
                        }
                        // Do nothing if profile was not set
                        rc::err::RemoveFitRahIncomingDmgError::DmgProfileNotSet(_) => (),
                    };
                }
            }
        }
        Ok(HCmdResp::NoData)
    }
}
