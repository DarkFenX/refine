use crate::{
    cmd::HCmdResp,
    shared::{HDmgProfile, HSecZone},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dmg: Option<HDmgProfile>,
}
impl HChangeSolCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into());
        }
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
        Ok(().into())
    }
}
