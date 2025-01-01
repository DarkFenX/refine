use crate::{cmd::HCmdResp, shared::HDmgProfile, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeDefaultIncomingDmg {
    dmg_profile: HDmgProfile,
}
impl HChangeDefaultIncomingDmg {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match core_sol.set_default_incoming_dmg((&self.dmg_profile).into()) {
            Ok(()) => Ok(().into()),
            Err(error) => Err(match error {
                rc::err::SetDefaultIncomingDmgError::EmDmgNegative(e) => HExecError::InvalidDmgProfileEm(e),
                rc::err::SetDefaultIncomingDmgError::ThermDmgNegative(e) => HExecError::InvalidDmgProfileTherm(e),
                rc::err::SetDefaultIncomingDmgError::KinDmgNegative(e) => HExecError::InvalidDmgProfileKin(e),
                rc::err::SetDefaultIncomingDmgError::ExplDmgNegative(e) => HExecError::InvalidDmgProfileExpl(e),
                rc::err::SetDefaultIncomingDmgError::TotalDmgNonPositive(e) => HExecError::InvalidDmgProfileTotal(e),
            }),
        }
    }
}
