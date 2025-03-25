use crate::util::HExecError;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::FitId,
}
impl HFitInfoId {
    pub(in crate::info::fit) fn mk_info(core_sol: &rc::SolarSystem, fit_id: &rc::FitId) -> Result<Self, HExecError> {
        let core_fit = match core_sol.get_fit(fit_id) {
            Ok(core_fit) => core_fit,
            Err(error) => match error {
                rc::err::GetFitError::FitNotFound(e) => return Err(HExecError::FitNotFoundPrimary(e)),
            },
        };
        let info = Self { id: core_fit.id };
        Ok(info)
    }
}
