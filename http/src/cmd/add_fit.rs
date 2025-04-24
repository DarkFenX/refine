use crate::{cmd::HFitIdResp, shared::HDpsProfile, util::HExecError};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddFitCmd {
    sec_status: Option<rc::SecStatus>,
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFitIdResp, HExecError> {
        let mut core_fit = core_sol.add_fit();
        if let Some(sec_status) = self.sec_status {
            core_fit.set_sec_status(sec_status).map_err(|error| match error {
                rc::err::SetFitSecStatusError::SecStatusError(e) => HExecError::InvalidSecStatus(e),
            })?;
        }
        if let Some(rah_incoming_dps) = &self.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps.try_into()?);
        }
        Ok(core_fit.into())
    }
}
