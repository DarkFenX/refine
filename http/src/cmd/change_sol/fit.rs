use crate::{cmd::HCmdResp, util::HExecError};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateFitCmd {}
impl HCreateFitCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HCmdResp {
        let core_fit = core_sol.add_fit();
        core_fit.into()
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
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
