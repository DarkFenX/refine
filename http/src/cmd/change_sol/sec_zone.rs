use crate::{cmd::HCmdResp, shared::HSecZone, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HSetSecZone {
    sec_zone: HSecZone,
}
impl HSetSecZone {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        core_sol.set_sec_zone((&self.sec_zone).into());
        Ok(().into())
    }
}
