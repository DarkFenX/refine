use crate::{shared::HDmgProfile, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFitCmd {
    rah_incoming_dmg: Option<HDmgProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolFitId, HExecError> {
        let fit_id = core_sol.add_fit().id;
        if let Some(rah_incoming_dmg) = &self.rah_incoming_dmg {
            if let Err(error) = core_sol.set_fit_rah_incoming_dmg(&fit_id, rah_incoming_dmg.into()) {
                match error {
                    rc::err::SetFitRahIncomingDmgError::EmDmgNegative(e) => {
                        core_sol.remove_fit(&fit_id).unwrap();
                        return Err(HExecError::InvalidDmgProfileEm(e));
                    }
                    rc::err::SetFitRahIncomingDmgError::ThermDmgNegative(e) => {
                        core_sol.remove_fit(&fit_id).unwrap();
                        return Err(HExecError::InvalidDmgProfileTherm(e));
                    }
                    rc::err::SetFitRahIncomingDmgError::KinDmgNegative(e) => {
                        core_sol.remove_fit(&fit_id).unwrap();
                        return Err(HExecError::InvalidDmgProfileKin(e));
                    }
                    rc::err::SetFitRahIncomingDmgError::ExplDmgNegative(e) => {
                        core_sol.remove_fit(&fit_id).unwrap();
                        return Err(HExecError::InvalidDmgProfileExpl(e));
                    }
                    rc::err::SetFitRahIncomingDmgError::FitNotFound(_) => panic!(),
                }
            };
        };
        Ok(fit_id)
    }
}
impl Default for HAddFitCmd {
    fn default() -> Self {
        Self { rah_incoming_dmg: None }
    }
}
