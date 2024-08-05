use crate::util::HExecResult;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolFitId,
}
impl HFitInfoId {
    pub(in crate::info::fit) fn mk_info(core_sol: &rc::SolarSystem, fit_id: &rc::SolFitId) -> HExecResult<Self> {
        let core_fit = core_sol.get_fit(fit_id)?;
        let info = Self { id: core_fit.id };
        Ok(info)
    }
}
