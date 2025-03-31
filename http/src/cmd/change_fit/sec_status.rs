use crate::{cmd::HCmdResp, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HSetSecStatusCmd {
    sec_status: rc::SecStatus,
}
impl HSetSecStatusCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        if let Err(error) = core_sol.set_fit_sec_status(fit_id, self.sec_status) {
            return Err(match error {
                rc::err::SetFitSecStatusError::SecStatusError(e) => HExecError::InvalidSecStatus(e),
                rc::err::SetFitSecStatusError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
            });
        }
        Ok(HCmdResp::NoData)
    }
}
