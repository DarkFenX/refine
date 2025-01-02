use crate::{shared::HDmgProfile, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSolCmd {
    default_incoming_dmg: Option<HDmgProfile>,
}
impl HAddSolCmd {
    pub(crate) fn execute(&self, src: rc::Src) -> Result<rc::SolarSystem, HExecError> {
        let mut core_sol = rc::SolarSystem::new(src);
        if let Some(default_incoming_dmg) = &self.default_incoming_dmg {
            if let Err(error) = core_sol.set_default_incoming_dmg(default_incoming_dmg.into()) {
                return Err(match error {
                    rc::err::SetDefaultIncomingDmgError::EmDmgNegative(e) => HExecError::InvalidDmgProfileEm(e),
                    rc::err::SetDefaultIncomingDmgError::ThermDmgNegative(e) => HExecError::InvalidDmgProfileTherm(e),
                    rc::err::SetDefaultIncomingDmgError::KinDmgNegative(e) => HExecError::InvalidDmgProfileKin(e),
                    rc::err::SetDefaultIncomingDmgError::ExplDmgNegative(e) => HExecError::InvalidDmgProfileExpl(e),
                    rc::err::SetDefaultIncomingDmgError::TotalDmgNonPositive(e) => {
                        HExecError::InvalidDmgProfileTotal(e)
                    }
                });
            }
        }
        Ok(core_sol)
    }
}
impl Default for HAddSolCmd {
    fn default() -> Self {
        Self {
            default_incoming_dmg: None,
        }
    }
}
