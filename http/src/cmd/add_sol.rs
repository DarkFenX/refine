use crate::{
    shared::{HDpsProfile, HSecZone},
    util::HExecError,
};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
}
impl HAddSolCmd {
    pub(crate) fn execute(&self, src: rc::Src) -> Result<rc::SolarSystem, HExecError> {
        let mut core_sol = rc::SolarSystem::new(src);
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
        Ok(core_sol)
    }
}
