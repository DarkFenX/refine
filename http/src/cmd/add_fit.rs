use crate::{shared::HDmgProfile, util::HExecError};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddFitCmd {
    rah_incoming_dmg: Option<HDmgProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::FitInfo, HExecError> {
        let fit_info = core_sol.add_fit();
        match &self.rah_incoming_dmg {
            Some(rah_incoming_dmg) => {
                if let Err(error) = core_sol.set_fit_rah_incoming_dmg(&fit_info.id, rah_incoming_dmg.into()) {
                    match error {
                        rc::err::SetFitRahIncomingDmgError::EmDmgNegative(e) => {
                            core_sol.remove_fit(&fit_info.id).unwrap();
                            return Err(HExecError::InvalidDmgProfileEm(e));
                        }
                        rc::err::SetFitRahIncomingDmgError::ThermDmgNegative(e) => {
                            core_sol.remove_fit(&fit_info.id).unwrap();
                            return Err(HExecError::InvalidDmgProfileTherm(e));
                        }
                        rc::err::SetFitRahIncomingDmgError::KinDmgNegative(e) => {
                            core_sol.remove_fit(&fit_info.id).unwrap();
                            return Err(HExecError::InvalidDmgProfileKin(e));
                        }
                        rc::err::SetFitRahIncomingDmgError::ExplDmgNegative(e) => {
                            core_sol.remove_fit(&fit_info.id).unwrap();
                            return Err(HExecError::InvalidDmgProfileExpl(e));
                        }
                        rc::err::SetFitRahIncomingDmgError::FitNotFound(_) => panic!(),
                    }
                }
            }
            None => return Ok(fit_info),
        }
        Ok(core_sol.get_fit(&fit_info.id).unwrap())
    }
}
