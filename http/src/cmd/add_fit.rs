use crate::{shared::HDpsProfile, util::HExecError};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddFitCmd {
    sec_status: Option<rc::SecStatus>,
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::FitInfo, HExecError> {
        let fit_info = core_sol.add_fit();
        if let Some(sec_status) = self.sec_status {
            if let Err(error) = core_sol.set_fit_sec_status(&fit_info.id, sec_status) {
                match error {
                    rc::err::SetFitSecStatusError::SecStatusError(e) => {
                        core_sol.remove_fit(&fit_info.id).unwrap();
                        return Err(HExecError::InvalidSecStatus(e));
                    }
                    rc::err::SetFitSecStatusError::FitNotFound(_) => panic!(),
                }
            }
        }
        if let Some(rah_incoming_dps) = &self.rah_incoming_dps {
            if let Err(error) = core_sol.set_fit_rah_incoming_dps(&fit_info.id, rah_incoming_dps.into()) {
                match error {
                    rc::err::SetFitRahIncomingDpsError::EmDpsNegative(e) => {
                        core_sol.remove_fit(&fit_info.id).unwrap();
                        return Err(HExecError::InvalidDpsProfileEm(e));
                    }
                    rc::err::SetFitRahIncomingDpsError::ThermDpsNegative(e) => {
                        core_sol.remove_fit(&fit_info.id).unwrap();
                        return Err(HExecError::InvalidDpsProfileTherm(e));
                    }
                    rc::err::SetFitRahIncomingDpsError::KinDpsNegative(e) => {
                        core_sol.remove_fit(&fit_info.id).unwrap();
                        return Err(HExecError::InvalidDpsProfileKin(e));
                    }
                    rc::err::SetFitRahIncomingDpsError::ExplDpsNegative(e) => {
                        core_sol.remove_fit(&fit_info.id).unwrap();
                        return Err(HExecError::InvalidDpsProfileExpl(e));
                    }
                    rc::err::SetFitRahIncomingDpsError::FitNotFound(_) => panic!(),
                }
            }
        }
        Ok(core_sol.get_fit(&fit_info.id).unwrap())
    }
}
