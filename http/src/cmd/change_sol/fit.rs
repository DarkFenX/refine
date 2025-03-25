use crate::{cmd::HCmdResp, util::HExecError};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
}
impl HDeleteFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match core_sol.remove_fit(&self.fit_id) {
            Ok(_) => Ok(HCmdResp::NoData),
            Err(error) => Err(match error {
                rc::err::RemoveFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
            }),
        }
    }
}
