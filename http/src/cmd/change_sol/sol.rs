use crate::{
    cmd::HCmdResp,
    shared::{HDpsProfile, HSecZone},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
}
impl HChangeSolCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into());
        }
        if let Some(default_incoming_dps) = &self.default_incoming_dps {
            if let Err(error) = core_sol.set_default_incoming_dps(default_incoming_dps.into()) {
                return Err(match error {
                    rc::err::SetDefaultIncomingDpsError::EmDpsNegative(e) => HExecError::InvalidDpsProfileEm(e),
                    rc::err::SetDefaultIncomingDpsError::ThermDpsNegative(e) => HExecError::InvalidDpsProfileTherm(e),
                    rc::err::SetDefaultIncomingDpsError::KinDpsNegative(e) => HExecError::InvalidDpsProfileKin(e),
                    rc::err::SetDefaultIncomingDpsError::ExplDpsNegative(e) => HExecError::InvalidDpsProfileExpl(e),
                    rc::err::SetDefaultIncomingDpsError::TotalDpsNonPositive(e) => {
                        HExecError::InvalidDpsProfileTotal(e)
                    }
                });
            }
        }
        Ok(().into())
    }
}
